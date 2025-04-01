// https://github.com/neknaj/cDom

export function elm(
    type: string,
    prop: { [key: string]: any },
    children: Node[]
): HTMLElement {
    const elm = document.createElement(type);
    const propKeys = Object.keys(prop);
    for (const key of propKeys) {
        if (key === "data") {
            for (const k of Object.keys(prop[key])) {
                elm.dataset[k] = prop[key][k];
            }
        } else {
            elm.setAttribute(key, prop[key]);
        }
    }
    for (const child of children) {
        elm.appendChild(child);
    }
    return elm;
}

export function textelm(text: string): Text {
    return document.createTextNode(text);
}

declare global {
    interface Element {
        Clear(): this;
        Add(child: Node): this;
        Proc<T extends HTMLElement>(func: (elm: T) => any): T;
        Replace(children: Node[]): this;
        Listen(
            type: string,
            listener: EventListenerOrEventListenerObject,
            options?: boolean | AddEventListenerOptions
        ): this;
        addProp(prop: { [key: string]: any }): this;
        addClass(name: string): this;
    }
}

Element.prototype.Clear = function (): Element {
    this.innerHTML = "";
    return this;
};

Element.prototype.Add = function (child: Node): Element {
    this.appendChild(child);
    return this;
};

Element.prototype.Proc = function <T extends HTMLElement>(func: (elm: T) => any): T {
    func(this as T);
    return this as T;
};

Element.prototype.Replace = function (children: Node[]) {
    this.replaceChildren(...children);
    return this;
};

Element.prototype.Listen = function (
    type: string,
    listener: EventListenerOrEventListenerObject,
    options?: boolean | AddEventListenerOptions
): Element {
    this.addEventListener(type, listener, options);
    return this;
};

Element.prototype.addProp = function (prop: { [key: string]: any }): Element {
    const propKeys = Object.keys(prop);
    for (const key of propKeys) {
        if (key === "data") {
            for (const k of Object.keys(prop[key])) {
                this.dataset[k] = prop[key][k];
            }
        } else {
            this.setAttribute(key, prop[key]);
        }
    }
    return this;
};

Element.prototype.addClass = function (name: string): Element {
    this.classList.add(name);
    return this;
};
