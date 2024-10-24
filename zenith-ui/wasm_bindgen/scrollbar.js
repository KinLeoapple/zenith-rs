export function scroll(el, always_on_top) {
    let ps = new PerfectScrollbar(el, {
        wheelSpeed: 2,
        wheelPropagation: false
    });

    const resize = () => {
        ps && ps.update();
    };

    if (always_on_top) {
        el.querySelector(".ps__rail-x").style.zIndex = "999999";
        el.querySelector(".ps__rail-y").style.zIndex = "999999";
    }
    window.addEventListener("resize", resize, false);
}