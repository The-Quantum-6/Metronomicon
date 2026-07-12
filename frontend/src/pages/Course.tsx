import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import Navbar from "../components/Navbar";
import ContributeAlert from "../components/ContributeAlert";
import Contribute, {type ContributionType} from "../components/Contribute";
import { apiUrl } from "../config";

const DISCLAIMER_SEEN = "metronomicon_policy_acknowledged"

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
  const [showAlert, setShowAlert] = useState(false);
  const [showContribute, setShowContribute] = useState(false);
  const [preselecteType, setPreselectedType] = useState<ContributionType | null>(null);


  useEffect(() => {
    fetch(apiUrl(`courses/${id}`))
      .then((r) => r.json())
      .then((data) => setCourse(data))
      .catch(console.error);
  }, [id]);

  const handleContribute = () => {
    if (localStorage.getItem(DISCLAIMER_SEEN)){
      setShowContribute(true);
    }
    else {
      setShowAlert(true)
    }
  }

  if (!course) return null;

return (
  <>
    <Navbar />
    <main className="max-w-7xl mx-auto px-4 sm:px-6 py-8">
    <div className="text-[#6B6B5A] hover:text-[#1A1F3A] transition-colors">
      <button
      onClick={() => navigate("/")}
      className="inline-flex items-center gap-1.5 py-4 font-sans"><svg className="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 10">
        <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M13 5H1m0 0 4 4M1 5l4-4"/>
        </svg>Back to courses</button>
        </div>
      <div className="pb-3 border-b border-[#F4F2EB]">
        <p className="font-mono text-m text-[#6B6B5A]">
          {course.code}
        </p>
        <div className="flex items-start justify-between gap-4 flex-wrap pb-4 border-b border-[#DAD8D6]">
          <h1 className="text-4xl font-semibold font-display text-[#1A1F3A]">
            {course.name}
          </h1>
          <div className="flex gap-2">
            <button onClick={ handleContribute }className="flex items-center px-4 py-2 rounded-lg bg-[#1A1F3A] text-white font-medium hover:opacity-90 transition-colors">
              + Contribute
            </button>
            <button onClick={() => alert("Coming soon..")}
            className="flex items-center gap-1.5 px-4 py-2 text-[#6B6B5A] rounded-lg bg-transparent border border-[#6B6B5A] hover:bg-gray-100 transition-colors">
              Report
            </button>
          </div>
        </div>
      </div>

      <div className="flex overflow-x-auto pt-3">
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

      {tab === "overview"  && 
      <div className="bg-white border border-[#DAD8D6] rounded-2xl p-5 mt-5 max-w-4xl w-full">
        <h2 className="font-bold font-display text-xl pb-2">About this course</h2>
        <p className="text-[#6B6B5A]">{course.content}</p>
      </div>
      }


      {tab === "resources" && 
      <div>
        <button onClick={() => { setPreselectedType("resource"); handleContribute();}}className="border border-dashed border-[#6B6B5A] rounded-lg px-4 py-2 text-[#6B6B5A] w-full hover:bg-gray-200">+ Add resources</button>
        </div>}
      {tab === "links"     && 
      <div>
        <button onClick={() => { setPreselectedType("link"); handleContribute();}}className="border border-dashed border-[#6B6B5A] rounded-lg px-4 py-2 text-[#6B6B5A] w-full hover:bg-gray-200">+ Add link</button>
        </div>}
      {tab === "projects"  && 
      <div>
        <button onClick={() => { setPreselectedType("project_idea"); handleContribute();}}className="border border-dashed border-[#6B6B5A] rounded-lg px-4 py-2 text-[#6B6B5A] w-full hover:bg-gray-200">+ Add project idea</button>
        </div>}
      {tab === "faq"       && 
      <div>
        <button onClick={() => { setPreselectedType("faq"); handleContribute();}}className="border border-dashed border-[#6B6B5A] rounded-lg px-4 py-2 text-[#6B6B5A] w-full hover:bg-gray-200">+ Add FAQ</button>
        </div>}

    </main>

    {showAlert && (
      <ContributeAlert onConfirm={() => {setShowAlert(false); setShowContribute(true)}} onCancel={() => setShowAlert(false)}
      />
    )}

    {showContribute && (
      <Contribute 
      preselected={preselecteType}
      onCancel={() => {setShowContribute(false); setPreselectedType(null)}}
        />
    )

    }


  </>
);
}