import { useState } from "react"

export type ContributionType = "resource" | "link" | "project_idea" | "faq"

interface ContributeModalProps {
  onCancel: () => void
  preselected?: ContributionType | null
}

const options: { id: ContributionType; label: string; description: string}[] = [
  { id: "resource", label: "Resource",      description: "Upload a file or document"},
  { id: "link",     label: "External Link", description: "Share a useful URL"},
  { id: "project_idea",  label: "Project Idea",  description: "Suggest a project idea"},
  { id: "faq",      label: "FAQ",           description: "Add a question and answer"},
]

export default function ContributeModal({ onCancel, preselected = null }: ContributeModalProps) {
  const [selected, setSelected] = useState<ContributionType | null>(preselected)
  const selectedOption = selected ? options.find((o) => o.id === selected) : null

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm bg-black/40 text-[#1A1F3A]">
      <div className="bg-white rounded-2xl p-7 max-w-4xl w-full mx-4 shadow-xl">

        {!selected ? (
          <>
            <h2 className="text-2xl font-semibold font-display mb-3">What would you like to contribute?</h2>
            <div className="grid grid-cols-2 gap-3 mb-6">
              {options.map((o) => (
                <button
                  key={o.id}
                  onClick={() => setSelected(o.id)}
                  className="flex items-center p-4 border border-[#F4F2EB] rounded-xl hover:bg-[#F4F2EB] transition-colors text-left"
                >
                  <p className="text-lg font-medium">{o.label}</p>
                </button>
              ))}
            </div>
            <div className="flex justify-end">
              <button
                onClick={onCancel}
                className="px-4 py-2 text-lg text-[#6B6B5A] border border-[#6B6B5A] rounded-lg hover:bg-gray-100 transition-colors"
              >
                Cancel
              </button>
            </div>
          </>
        ) : (
          <>
            <button
              onClick={() => setSelected(null)}
              className="text-lg text-[#6B6B5A] mb-4 hover:text-[#1A1F3A] transition-colors"><svg className="w-6 h-6" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 10">
                <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="1.5" d="M13 5H1m0 0 4 4M1 5l4-4"/></svg>
              </button>
            <h2 className="text-2xl font-semibold font-display mb-6">
              {selectedOption?.description}
            </h2>
            <div className="bg-[#F4F2EB] rounded-xl p-4 mb-6">
            </div>
            <div className="flex gap-2 justify-end">
              <button onClick={() => alert("contribution sent for review. Thank you!")}
              className="p-2 text-lg bg-[#1A1F3A] text-white rounded-lg hover:opacity-90 transition-colors">Send for review</button>
              <button
                onClick={onCancel}
                className="p-2 text-lg text-[#6B6B5A] border border-[#6B6B5A] rounded-lg hover:bg-gray-100 transition-colors"
              >
                Cancel
              </button>
            </div>
          </>
        )}

      </div>
    </div>
  )
}