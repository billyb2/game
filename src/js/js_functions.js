export function screen_width() {
    return window.innerWidth;
}

export function screen_height() {
    return window.innerHeight;
}

export function js_get_data(key) {
    return localStorage.getItem(key);
}

export function js_write_data(key, value) {
    localStorage.setItem(key, value);

}
