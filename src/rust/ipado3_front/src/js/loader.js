'use strict'

function load_app({
    APP,
    fadeoutFrom,
    fadeoutDelaySecs,
    fadeoutDurationSecs,
    fadeoutTiming,
}) {
    fadeoutFrom ||= 'rgba(128, 128, 128, 0.9)'
    fadeoutDelaySecs ||= 0
    fadeoutDurationSecs ||= 1
    // fadeoutTiming ||= 'ease-in' // https://developer.mozilla.org/en-US/docs/Web/CSS/transition-timing-function
    fadeoutTiming ||= 'linear' // https://developer.mozilla.org/en-US/docs/Web/CSS/transition-timing-function
    window.loaded = function () {
        // console.log('did loaded')
        const el = document.getElementById('splash');
        if (el) {  
            const fadeout = () => {
                // console.log('will fadeout')
                el.style.opacity = '0'
                const remove = () => {
                    // console.log('will remove')
                    el.remove()
                }
                setTimeout(remove, fadeoutDurationSecs * 1000)
            }
            if (!fadeoutDelaySecs) { 
                fadeout()
            } else {
                setTimeout(fadeout, fadeoutDelaySecs * 1000)
            }
        } 
    }
    {
        const el = document.createElement('style')
/* https://loading.io/css/ */
        el.innerHTML = `
            .lds-spinner {
              color: official;
              display: inline-block;
              position: relative;
              width: 80px;
              height: 80px;
            }
            .lds-spinner div {
              transform-origin: 40px 40px;
              animation: lds-spinner 1.2s linear infinite;
            }
            .lds-spinner div:after {
              content: " ";
              display: block;
              position: absolute;
              top: 3px;
              left: 37px;
              width: 6px;
              height: 18px;
              border-radius: 20%;
              background: #fff;
            }
            .lds-spinner div:nth-child(1) {
              transform: rotate(0deg);
              animation-delay: -1.1s;
            }
            .lds-spinner div:nth-child(2) {
              transform: rotate(30deg);
              animation-delay: -1s;
            }
            .lds-spinner div:nth-child(3) {
              transform: rotate(60deg);
              animation-delay: -0.9s;
            }
            .lds-spinner div:nth-child(4) {
              transform: rotate(90deg);
              animation-delay: -0.8s;
            }
            .lds-spinner div:nth-child(5) {
              transform: rotate(120deg);
              animation-delay: -0.7s;
            }
            .lds-spinner div:nth-child(6) {
              transform: rotate(150deg);
              animation-delay: -0.6s;
            }
            .lds-spinner div:nth-child(7) {
              transform: rotate(180deg);
              animation-delay: -0.5s;
            }
            .lds-spinner div:nth-child(8) {
              transform: rotate(210deg);
              animation-delay: -0.4s;
            }
            .lds-spinner div:nth-child(9) {
              transform: rotate(240deg);
              animation-delay: -0.3s;
            }
            .lds-spinner div:nth-child(10) {
              transform: rotate(270deg);
              animation-delay: -0.2s;
            }
            .lds-spinner div:nth-child(11) {
              transform: rotate(300deg);
              animation-delay: -0.1s;
            }
            .lds-spinner div:nth-child(12) {
              transform: rotate(330deg);
              animation-delay: 0s;
            }
            @keyframes lds-spinner {
              0% {
                opacity: 1;
              }
              100% {
                opacity: 0;
              }
            }
        `
        document.head.appendChild(el)
    }
    {
        const el = document.createElement('div')
        const transition = `transition: opacity ${fadeoutDurationSecs}s ${fadeoutTiming}`
        el.innerHTML = `
            <div id="splash" style="
                position: fixed; 
                top: 0; left: 0; 
                z-index: 1000; 
                width: 100vw; 
                height: 100vh; 
                display: flex; 
                justify-content: center; 
                align-items: center; 
                opacity: 1;
                background-color: ${fadeoutFrom}; 
                ${transition}; 
                -moz-${transition}; 
                -webkit-${transition};
                -o-${transition};
            ">
                <div class="lds-spinner"><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div><div></div></div>
            </div>
        `
        document.body.appendChild(el.firstElementChild)
    }
    {
        const fragment = document.createDocumentFragment()
        {
            const el = document.createElement('script')
            el.type = 'module'
            el.innerText = `
                import init, * as wasm from './pkg/${APP}_front.js';
                await init('./pkg/${APP}_front_bg.wasm');
                window.wasm = wasm;
            `
            fragment.appendChild(el)
        }
        document.head.appendChild(fragment)
    }
}

export { load_app }
