import Navbar from "../components/Navbar";
import Footer from "../components/Footer";

export default function About() {
  return (
    <div className="min-h-screen flex flex-col bg-surface-dark text-text">
      <Navbar />
      <main className="container mx-auto max-w-3xl p-6 flex-1">
        <h1 className="text-2xl font-bold text-primary mb-4">About</h1>
        <p className="text-text-secondary mb-4">
          Metronomicon is a course resource hub built by students, for students.
          A single, lasting place to share the tips, links, and resources that
          actually helped you — organized by course.
        </p>
        <p className="text-text-secondary">
          Anyone can contribute. Everything is reviewed before it goes live, and
          contributors stay anonymous.
        </p>
      </main>
      <Footer />
    </div>
  );
}
