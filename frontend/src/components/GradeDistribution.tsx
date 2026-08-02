import { useEffect, useState } from "react";
import { apiUrl } from "../config";
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
  //plugins,
  //scales,
  //ChartOptions,
  //ChartData
} from 'chart.js';
import { Bar } from 'react-chartjs-2';
import { Label } from "@navikt/ds-react";

ChartJS.register(
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend
);

interface Props {
  courseCode: string;
}

type SemesterGrade = {
  year: number;
  semester: "autumn" | "spring";
  candidates: number;
  distribution: Record<string, number>;
  averageGrade: string | null;
};

const grades = ["A", "B", "C", "D", "E", "F"];
const semesters: ("autumn" | "spring")[] = ["autumn", "spring"]

export default function GradeOverview({ courseCode }: Props) {
  const [data, setData] = useState<SemesterGrade[]>([]);
  const [selectedYear, setSelectedYear] = useState<number | null>(null);
  const [selectedSemester, setSelectedSemester] = useState<"autumn" | "spring" | null>(null);

  useEffect(() => {
    fetch(apiUrl(`grades/${courseCode}`))
      .then((r) => r.json())
      .then((res) => {
        const semesterResult: SemesterGrade[] = res.grades ? res.grades.semesters : [];
        setData(semesterResult);
        if (semesterResult.length > 0) {
          const latest = semesterResult[semesterResult.length-1];
          setSelectedYear(latest.year);
          setSelectedSemester(latest.semester);
        } else {
          console.log("No grade data found for", courseCode);
        }
      }) 
  }, [courseCode]);

  const years = data.map((d) => d.year).filter((y, i, arr) => arr.indexOf(y) === i);
  const current = data.find((d) => d.year === selectedYear && d.semester === selectedSemester);

  if (!data.length) return null;
  const chartData = {
    labels: grades,
    datasets: [{
      label: "Candidates",
      data: grades.map((g) => current?.distribution[g] ?? 0),
      backgroundColor: "#1A1F3A",
      hoverBackgroundColor: "#5e6275",
      borderWidth: 1,
    },
  ],
};

const chartOptions = {
  scales: {
    y: {
      beginAtZero: true,
      precision: 0,
      stepSize: 1,
    },
  },
  plugins: {
    legend: {
      display: false,
    },
  },
};

return (
    <div className="bg-white border border-[#DAD8D6] rounded-2xl p-5 max-w-4xl w-full">
      <h2 className="font-bold font-display text-xl pb-4">Grade distribution</h2>

      <div className="flex items-center gap-5 mb-6">
        <div className="flex overflow-hidden gap-2">
          {semesters.map((s) => (
            <button key={s} onClick={() => setSelectedSemester(s)}
            className={`rounded-lg border border-[#DAD8D6] px-2 py-1 text-base transition-colors ${selectedSemester === s ? "bg-[#1A1F3A] text-white" : "text-[#6B6B5A] hover:bg-gray-100"}`}>
              {s === "spring" ? "Spring" : "Autumn"}
            </button>
          ))}
        </div>

        <select value={selectedYear ?? ""}
        onChange={(e) => setSelectedYear(Number(e.target.value))}
        className="border border-[#DAD8D6] rounded-lg p-1 text-base focus:outline-none">
          {years.map((y) => (
            <option key={y} value={y}>{y}</option>))}
        </select>
      </div>
      

      {current && (
        <>
        <div className="flex justify-between items-center mb-6">
          <div>
          <p className="text-sm text-[#6B6B5A]">{current.candidates} candidates</p>
        </div>
        <div className="text-right">
          <p className="text-sm font-semibold text-[#1A1F3A]">Average: {current.averageGrade ?? "-"}</p>
        </div>
        </div>
        <Bar data={chartData} options={chartOptions}/>
        <p className="text-sm text-[#6B6B5A] mt-3">Source: <a href="https://karakterweb.no" className="underline">karaterweb.no</a></p>
        </>
      )}
        </div>
      )
}