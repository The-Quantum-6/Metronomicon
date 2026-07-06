import { useState } from "react";

interface ContributeAlertProps {
    onConfirm: () => void
    onCancel: () => void
}

const DISCLAIMER_SEEN = "metronomicon_policy_acknowledged"

export default function ContributeAlert ({ onConfirm, onCancel }: ContributeAlertProps) {
    const [dontShow, setDontShow] = useState(false)

    const handleConfirm = () => {
        if (dontShow) {
            localStorage.setItem(DISCLAIMER_SEEN, "true")
        }
        onConfirm()
    }
    return(
        <div className="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm bg-black/40 text-[#1A1F3A]">
            <div className="bg-white rounded-2xl p-7 max-w-4xl w-full mx-4 shadow-xl">
                <h2 className="text-3xl font-semibold font-display mb-3">Before you contribute</h2>
                <p className="text-xl mb-5 bg-[#ECE9DF] rounded-xl p-4">Everything you post is <b>anonymous to other users</b>, but moderators must approve your contributions and will therefore be able to see your name.
                    Once you contribute, <b>you cannot request removal</b> of your material. In case of sensitive information, contact us for immediate removal.</p>
            <label className="flex items-center gap-2 text-xl text-[#6B6B5A] mb-6">
                <input type="checkbox" checked={dontShow} onChange={() => setDontShow(!dontShow)} /> Don't show this again
            </label>
            <div className="flex gap-3 justify-end">
                <button onClick={onCancel} className="px-4 py-2 text-xl text-[#6B6B5A] border border-[#6B6B5A] rounded-lg hover:bg-gray-100 transition-colors"> Cancel</button>
                <button onClick={handleConfirm} className="px-4 py-2 text-xl bg-[#1A1F3A] text-white rounded-lg hover:opacity-90 transition-opacity">I understand - continue</button>
            </div>
            </div>
        </div>
    )
}

