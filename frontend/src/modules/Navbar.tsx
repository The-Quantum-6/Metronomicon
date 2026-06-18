function Navbar() {
  return (
    <nav style={{ display: 'flex', gap: '1rem', padding: '1rem', background: '#222' }}>
      <a href="/" style={{ color: '#fff' }}>Home</a>
      <a href="/about" style={{ color: '#fff' }}>About</a>
      <a href="/contact" style={{ color: '#fff' }}>Contact</a>
    </nav>
  )
}

export default Navbar