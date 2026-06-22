import Navbar from "../components/Navbar";
import { Fragment, useEffect, useState } from "react";
import type { ReactNode } from "react";
import { Link } from "react-router-dom";
import { apiUrl } from "../config";

type Course = {
  id: string;
  name: string;
  code: string;
};

// Replace each "[image]" token in text with a deterministic image URL.
// Using a stable seed avoids refetching/flicker on re-renders.
function renderWithImages(text: string, seedBase: string): ReactNode[] {
  return text.split("[image]").reduce<ReactNode[]>((acc, part, i) => {
    if (i > 0) {
      const seed = `${seedBase}-${i}`;
      acc.push(
        <img
          key={`img-${i}`}
          src={`https://picsum.photos/seed/${seed}/120/80`}
          alt=""
          aria-hidden="true"
          loading="lazy"
          decoding="async"
          className="inline-block align-middle rounded mx-1"
          width={120}
          height={80}
        />,
      );
    }
    if (part) acc.push(<Fragment key={`txt-${i}`}>{part}</Fragment>);
    return acc;
  }, []);
}

function Home() {
  const [courses, setCourses] = useState<Course[] | null>(null);

  useEffect(() => {
    fetch(apiUrl("courses"))
      .then((r) => r.json())
      .then((data) => setCourses(data))
      .catch(() => setCourses([]));
  }, []);

  return (
    <div className="min-h-screen bg-gray-800 text-white">
      <Navbar />
      <header className="container mx-auto p-6 border-b border-gray-700">
        <h1 className="text-4xl font-bold text-purple-400">Metronomicon</h1>
        <p className="text-gray-400 mt-2">A simple course prototype</p>
      </header>

      <main className="container mx-auto p-6">
        <h2 className="text-2xl mb-6 text-white">Courses</h2>
        {courses === null ? (
          <div className="text-gray-400">Loading...</div>
        ) : courses.length === 0 ? (
          <div className="text-gray-400">No courses yet.</div>
        ) : (
          <ul className="space-y-3">
            {courses.map((c) => (
              <li key={c.id}>
                <Link to={`/courses/${c.id}`} className="text-purple-400 hover:text-purple-300 transition block p-3 rounded hover:bg-gray-700">
                  {renderWithImages(c.name)} <span className="text-gray-500">({c.code})</span>
                </Link>
              </li>
            ))}
          </ul>
        )}
      </main>
    </div>
  );
}

export default Home;
