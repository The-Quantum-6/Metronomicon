import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import Navbar from "../components/Navbar";
import { apiUrl } from "../config";

type Course = {
  id: string;
  name: string;
  code: string;
  content?: string | null;
};

type CourseTab = "overview" | "resources" | "links" | "projects" | "faq";

const tabs = [
  { id: "overview"  as CourseTab, label: "Overview" },
  { id: "resources" as CourseTab, label: "Resources" },
  { id: "links"     as CourseTab, label: "Links" },
  { id: "projects"  as CourseTab, label: "Projects" },
  { id: "faq"       as CourseTab, label: "FAQ" },
];

export default function Course() {
  const navigate = useNavigate();
  const { id } = useParams();
  const [course, setCourse] = useState<Course | null>(null);
  const [tab, setTab] = useState<CourseTab>("overview");

  useEffect(() => {
    fetch(apiUrl(`courses/${id}`))
      .then((r) => r.json())
      .then((data) => setCourse(data))
      .catch(console.error);
  }, [id]);

  if (!course) return null;

return (
  <>
    <Navbar />
    <main className="max-w-5xl mx-auto px-4 sm:px-6 py-8">

      <button
        onClick={() => navigate("/")}
        className="flex items-center gap-2 py-4 font-sans text-[#6B6B5A] hover:text-[#1A1F3A] transition-colors">
        ← Back to courses
      </button>

      <div className="mb-7 pb-6 border-b border-[#F4F2EB]">
        <p className="font-mono text-sm py-2 text-[#6B6B5A]">
          {course.code}
        </p>
        <div className="flex items-start justify-between gap-4 flex-wrap">
          <h1 className="text-4xl font-semibold font-display text-[#1A1F3A]">
            {course.name}
          </h1>
          <div className="flex gap-2">
            <button className="flex items-center gap-1.5 px-4 py-2 rounded-lg bg-[#1A1F3A] text-white font-medium hover:bg-gray-200 transition-colors">
              + Contribute
            </button>
            <button className="flex items-center gap-1.5 px-4 py-2 text-[#6B6B5A] rounded-lg bg-transparent border border-[#6B6B5A] hover:bg-gray-200 transition-colors">
              + Report
            </button>
          </div>
        </div>
      </div>

      <div className="flex overflow-x-auto">
        {tabs.map((t) => (
          <button
            key={t.id}
            onClick={() => setTab(t.id)}
            className={`px-4 py-2 font-sans transition-colors rounded-lg
              ${tab === t.id ? "text-[#1A1F3A] bg-[#E3E1DD] border-[#F4F2EB]"
                : "text-[#6B6B5A] rounded-lg hover:bg-gray-200"}`}>
            {t.label}
          </button>
        ))}
      </div>

      {tab === "overview"  && <div>Overview</div>}
      {tab === "resources" && <div>Resources</div>}
      {tab === "links"     && <div>Links</div>}
      {tab === "projects"  && <div>Projects</div>}
      {tab === "faq"       && <div>FAQ</div>}

    </main>
  </>
);
}