import { useEffect, useState } from "react";
import { useNavigate, useParams} from "react-router-dom";
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
  { id: "overview"  as CourseTab, label: "Overview"},
  { id: "resources" as CourseTab, label: "Resources"},
  { id: "links"     as CourseTab, label: "Links"},
  { id: "projects"  as CourseTab, label: "Projects"},
  { id: "faq"       as CourseTab, label: "FAQ"},
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

  if (!course) return;

  return (
    <>
      <Navbar />
      <main>
        <button onClick={() => navigate("/")}>Back to courses</button>
        <div>
          <p>{course.code}</p>
          <div>
            <h1>{course.name}</h1>
            <div>
              <button>Contribute</button>
              <button>Report</button>
            </div>
          </div>
        </div>

        <div>
          {tabs.map((t) => (
            <button key={t.id} onClick={() => setTab(t.id)}>
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