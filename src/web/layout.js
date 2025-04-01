import { elm } from './cdom.js';

// https://github.com/neknaj/webSplitLayout

function setResizer (container,node,callback=()=>{}) {
    const normalizeProp = (arr)=>{
        const adj = 100/arr.reduce((s,x)=>{return s+x});
        return arr.map((x)=>{return x*adj})
    }
    const p = container.dataset.proportion.split(":").map((x)=>{return Number(x);});
    node[1] = normalizeProp(p);
    for (const i in node[1]) {
        container.querySelectorAll(":scope > .resizer_content")[i].style.flexBasis = `${(node[1][i])}%`;
    }
    container.dataset.proportion = node[1].join(":");
    const type = container.dataset.type=="v";
    Array.from(container.querySelectorAll(":scope > .resizer_splitter")).map((splitter,i)=>{
        splitter.addEventListener("pointerdown",(e)=>{
            const rect = container.getBoundingClientRect();
            const rects = type?rect.height:rect.width;
            const rectx = type?rect.y:rect.x;
            const resizer = container.querySelector(":scope > .resizer_splitter").getBoundingClientRect();
            const resizerW = type?resizer.height:resizer.width;
            const resize1 = (e)=>{
                const percent = container.dataset.proportion.split(":").map((x)=>{return Number(x);});
                const ex = type?e.y:e.x;
                const width = rects - resizerW*(percent.length-1);
                const barprop = Math.min(Math.max((ex-rectx)/rects,0.0001),0.9999);
                const left = percent.slice(0,i+1);
                const right = percent.slice(i+1);
                const leftadj = left.reduce((s,x)=>{return s+x},0)==0?1:1/left.reduce((s,x)=>{return s+x},0);
                const rightadj = right.reduce((s,x)=>{return s+x},0)==0?1:1/right.reduce((s,x)=>{return s+x},0);
                const newpercent = normalizeProp(left.map((x)=>{return x*barprop*leftadj*100}).concat(right.map((x)=>{return x*(1-barprop)*rightadj*100})).map((x)=>{return x<0.01?1:x;}));
                node[1] = newpercent;
                container.dataset.proportion = newpercent.join(":");
                for (const i in newpercent) {
                    container.querySelectorAll(":scope > .resizer_content")[i].style.flexBasis = `${(newpercent[i])}%`;
                }
                callback();
                container.dispatchEvent(new CustomEvent("resize"));
            }
            const resize2 = (e)=>{
                const percent = container.dataset.proportion.split(":").map((x)=>{return Number(x);});
                const ex = type?e.y:e.x;
                const width = rects - resizerW*(percent.length-1);
                const left = percent.slice(0,i+1-1);
                const right = percent.slice(i+1+1);
                const min = left.reduce((s,x)=>{return s+x},0)*rects/100+resizerW*i;
                const max = rects - (right.reduce((s,x)=>{return s+x},0)*rects/100+resizerW*(percent.length-i-2));
                const newx = Math.min(Math.max(ex-rectx,min),max);
                if (max-newx<1|newx-min<1) {
                    resize1(e);callback();return;
                }
                const newpercent = normalizeProp([].concat(left,[(newx-min)*100/width],[(max-newx)*100/width],right));
                node[1] = newpercent;
                container.dataset.proportion = newpercent.join(":");
                for (const i in newpercent) {
                    container.querySelectorAll(":scope > .resizer_content")[i].style.flexBasis = `${(newpercent[i])}%`;
                }
                callback();
                container.dispatchEvent(new CustomEvent("resize"));
            }
            document.addEventListener("pointermove",resize2,false);
            document.addEventListener("pointerup",()=>{
                document.removeEventListener("pointermove",resize2,false);
                container.dispatchEvent(new CustomEvent("resizeend"));
            },false);
            e.target.setPointerCapture(e.pointerId);
        });
    })
    container.dispatchEvent(new CustomEvent("resizestart"));
    return container;
}

function initlayout(elm,layout) {
    elm.className = "layout_root";
    elm.replaceChildren(makeLayoutDOM(layout,"splitlayout"));
    elm.dispatchEvent(new CustomEvent("doneinitlayout"));
}
function makeLayoutDOM(node,pid) {
    var children = [];
    for (const i in node[2]) {
        if (node[2]?.[i]?.[0]=="h"||node[2]?.[i]?.[0]=="v") {
            children.push(elm("div",{class:"resizer_content"},[makeLayoutDOM(node[2][i],`${pid}_${i}`)]));
        }
        else if (node[2]?.[i]?.[0]=="c") {
            children.push(elm("div",{class:"resizer_content"},[contentarea(node[2][i][1],node[2][i])]));
        }
        else {
            console.log("")
            node[2][i] = ["c","empty"]
            children.push(elm("div",{class:"resizer_content"},[contentarea(node[2][i][1],node[2][i])]));
        }
        if (i<node[2].length-1) {
            children.push(elm("div",{class:"resizer_splitter"},[]));
        }
    }
    return setResizer(elm("div",{data:{proportion:node[1].join(":"),id:pid,type:node[0]},class:`resizer_container`},children),node);
}

const contentarea = (name,node)=>{
    return elm("div",{class:"layoutcontentarea"},[
        elm("div",{class:"layoutcontent",id:name},[]),
    ])
}

export { initlayout };