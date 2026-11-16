"""Saved source controls work without rerunning the plugin, in every existing preset."""
from helpers import *
import json


def test_controls(approval):
    report={}
    for layout in ('drawers','two-door'):
      for construction in ('overlay','inset'):
       for style in ('slab','shaker','raised-panel'):
        key=f'{layout}-{construction}-{style}'
        source=ROOT/'dist'/f'{key}.sc'
        assert source.exists(), 'Run test_base first'
        checks=[]
        states=[dict(show_hardware=0),dict(use_knobs=1),dict(show_fronts=0),dict(center_hardware=0,hardware_top_offset=90,hardware_x_offset=15),dict(plinth=140,toe_kick_setback=110)]
        if layout=='two-door':states += [dict(shelf_count=n) for n in (0,2,3,6)]
        for index,values in enumerate(states):
          target=ROOT/'dist'/f'{key}-controls-{index}.sc'
          lengths={'hardware_top_offset','hardware_x_offset','plinth','toe_kick_setback'}
          changes={'SDH_Base_'+k:str(v)+(' mm' if k in lengths else '') for k,v in values.items()}
          replies=cli([{'command':'parameters.change','params':{'changes':changes}},MEASURE],'--in',str(source),'--save',str(target))
          bb=boxes(replies[1]);names=list(bb)
          hardware=[k for k in names if 'Hardware' in k]
          fronts=[k for k in names if ' Drawer Front' in k or ' Door L' in k or ' Door R' in k]
          if values.get('show_fronts')==0: assert not fronts and not hardware
          elif values.get('show_hardware')==0: assert fronts and not hardware
          else: assert fronts and hardware
          if values.get('use_knobs')==1: assert all('Knob' in k for k in hardware)
          elif hardware: assert all('Pull' in k for k in hardware)
          if 'shelf_count' in values: assert sum('Adjustable shelf' in k for k in names)==values['shelf_count']
          if 'toe_kick_setback' in values:
            assert bb['SDH Base Plinth'][1]==round(-609.6/2+110,3)
            assert bb['SDH Base Plinth'][5]==140
          reopened=cli([MEASURE],'--in',str(target))[0]
          assert bb==boxes(reopened)
          checks.append(dict(values=values,bodies=len(bb),hardware=len(hardware),reopened=True))
        report[key]=checks
    # Adding a second cabinet must not restore the first one's Default configuration values.
    first=ROOT/'dist/drawers-overlay-slab.sc'
    r=cli([{'command':'parameters.change','params':{'changes':{'SDH_Base_show_fronts':'0'}}},plugin(approval,'base',dict(name='Second',layout='two-door',show_fronts=False,shelf_count=0)),MEASURE],'--in',str(first))
    names=list(boxes(r[-1]))
    assert not any('Drawer Front' in k or ' Door L' in k or ' Door R' in k or 'Hardware' in k or 'Adjustable shelf' in k for k in names)
    report['second_cabinet_preserves_edits']=True
    (ROOT/'dist/shared-controls-acceptance.json').write_text(json.dumps(report,indent=2))
