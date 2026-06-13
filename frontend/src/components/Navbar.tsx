const Navbar = () => {
    return (
        <nav className="border-b border-gray-200 px-6 h-14 flex items-center justify-between">
            <a href="/" className="font-bold text-lg">Metronomicon</a>
            <button onClick={() => alert("under construction...")} className="border border-gray-300 rounded-lg px-5 py-2">Log in</button>
        </nav>

    )
}
export default Navbar
