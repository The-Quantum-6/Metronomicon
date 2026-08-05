interface ContributeAlertProps {
  onConfirm: () => void;
  onCancel: () => void;
}

export default function ContributeAlert({ onConfirm, onCancel }: ContributeAlertProps) {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm bg-black/40 text-[#1A1F3A]">
      <div className="bg-white rounded-2xl p-7 max-w-4xl w-full mx-4 shadow-xl">
        <h2 className="text-2xl font-semibold font-display mb-3">Before you contribute</h2>
        <div className="text-xl mb-6 bg-[#ECE9DF] rounded-lg p-4 space-y-3">
          <p>
            Everything you post is <b>anonymous to other users</b>, but moderators must approve your
            contributions and will therefore be able to see your name. Once you contribute,{" "}
            <b>you cannot request removal</b> of your material. For sensitive information, contact us
            for immediate removal.
          </p>
          <p>
            <b>Do not post anything copyrighted or illegal.</b> Otherwise, follow "Kardemomme-loven":
            don't disturb others, be kind, and otherwise do as you please.
          </p>
        </div>
        <div className="flex gap-3 justify-end">
          <button
            onClick={onCancel}
            className="px-4 py-2 text-lg text-[#6B6B5A] border border-[#6B6B5A] rounded-lg hover:bg-gray-100 transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={onConfirm}
            className="px-4 py-2 text-lg bg-[#1A1F3A] text-white rounded-lg hover:opacity-90 transition-opacity"
          >
            I understand - continue
          </button>
        </div>
      </div>
    </div>
  );
}
