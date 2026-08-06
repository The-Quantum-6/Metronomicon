export function toast(message: string, ok = true) {
    const box = document.createElement("div")
    box.textContent = message
    box.style.cssText = `
        position: fixed; bottom: 1.5rem; right: 1.5rem; z-index: 100;
        background: #fff; color: #1A1F3A;
        padding: 0.75rem 1.25rem; border-radius: 0.75rem;
        box-shadow: 0 10px 25px rgba(0,0,0,0.15);
        border-left: 4px solid ${ok ? "#1A1F3A" : "#B3261E"};
    `
    document.body.appendChild(box)
    setTimeout(() => box.remove(), 2500)
}
