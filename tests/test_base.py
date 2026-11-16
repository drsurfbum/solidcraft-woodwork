"""Twelve source templates, dimensional invariants, editable 1–4 drawers and reopen."""
from helpers import *
import json


def validate(measure, layout, construction, style, count=3, w=609.6, h=876.3, d=609.6):
    all_bodies=measure['bodies'];bb=boxes(measure)
    assert all(b['volume_mm3']>0 for b in all_bodies)
    assert abs(min(v[0] for v in bb.values())+w/2)<.001
    assert abs(max(v[3] for v in bb.values())-w/2)<.001
    assert abs(max(v[5] for v in bb.values())-h)<.001
    front_y=-d/2-19.05-(21.05 if construction=='overlay' else 0)
    assert abs(min(v[1] for k,v in bb.items() if "Hardware" not in k)-front_y)<.001
    if layout=='drawers':
        names=[k for k in bb if ' Drawer Front' in k]
        assert len(names)==count*(1 if style=='slab' else 5),(style,len(names),count)
        assert len([k for k in bb if ' Drawer Left' in k])==count
        rows=sorted(set(bb[k][2] for k in names if style=='slab' or 'Stile L' in k))
        assert len(rows)==count
        # The inset fronts sit inside the frame with a real gap on all four sides.
        if construction=='inset':
            assert all(abs(bb[k][0]-(-w/2+38.1+2))<.001 for k in names if style=='slab' or 'Stile L' in k)
            rails=sorted(v for k,v in bb.items() if ' Face frame bottom' in k)
            assert len(rails)==count
            for bottom in rows:assert any(abs(bottom-v[5]-2)<.002 for v in rails)
    else:
        assert len([k for k in bb if ' Door L' in k])==(1 if style=='slab' else 5)
        assert len([k for k in bb if ' Door R' in k])==(1 if style=='slab' else 5)
    # Shaker is recessed; raised-panel field comes forward to the frame face.
    panels=[v for k,v in bb.items() if ' Panel' in k and (' Front' in k or ' Door ' in k)]
    if style!='slab':
        assert panels
        assert all(abs(v[1]-front_y-(6.35 if style=='shaker' else 0))<.002 for v in panels)
    return {'bodies':len(all_bodies),'width':w,'height':h,'carcass_depth':d,'drawer_count':count if layout=='drawers' else None}


def test_base_matrix(approval):
    results={}
    for layout in ('drawers','two-door'):
        for construction in ('overlay','inset'):
            for style in ('slab','shaker','raised-panel'):
                key=f'{layout}-{construction}-{style}'
                source=ROOT/'dist'/f'{key}.sc'
                generated=cli([plugin(approval,'base',dict(name='SDH Base',layout=layout,construction=construction,style=style)),MEASURE], '--save',str(source))
                checks=[validate(generated[1],layout,construction,style)]
                # Count is an ordinary source parameter; never rerun the generator to resize.
                for count in ((1,2,3,4) if layout=='drawers' else (3,)):
                    changed={'SDH_Base_width':'914.4 mm','SDH_Base_height':'965.2 mm','SDH_Base_depth':'660.4 mm'}
                    if layout=='drawers':changed['SDH_Base_drawer_count']=str(count)
                    target=ROOT/'dist'/f'{key}-edited.sc'
                    flex=cli([{'command':'parameters.change','params':{'changes':changed}},MEASURE],'--in',str(source),'--save',str(target))
                    checks.append(validate(flex[1],layout,construction,style,count,914.4,965.2,660.4))
                    reopened=cli([MEASURE],'--in',str(target))
                    assert boxes(reopened[0])==boxes(flex[1])
                results[key]=checks
    (ROOT/'dist/base-acceptance.json').write_text(json.dumps(results,indent=2))
