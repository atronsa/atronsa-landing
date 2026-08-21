"use client";

import { useRouter } from "next/navigation";

const PROJECT_TYPE_LABELS: Record<string, string> = {
  personal: "Personal Website",
  business: "Company Website",
  revamp: "Website Revamp",
  other: "Other",
};

const truncateMessage = (message: string) => {
  const words = message.split(/\s+/);
  const firstThree = words.slice(0, 3).join(" ");
  return words.length > 3 ? `${firstThree}...` : firstThree;
};

interface Message {
  id: number;
  name: string;
  email: string;
  phoneNumber: string;
  projectType: string | null;
  message: string;
  createdAt: Date;
}

export default function MessageTable({ messages }: { messages: Message[] }) {
  const router = useRouter();

  return (
    <tbody>
      {messages.map((m) => (
        <tr
          key={m.id}
          className="cursor-pointer border-b border-white/5 text-[11px] text-white/60 transition-colors last:border-0 hover:bg-white/4 sm:text-[12px]"
          onClick={() => router.push(`/admin/${m.id}`)}
        >
          <td colSpan={6} className="p-0 md:hidden">
            <div className="px-4 py-4">
              <div className="flex items-center justify-between gap-3">
                <span className="font-poppins text-sm text-white/90">
                  {m.name}
                </span>
                <span className="shrink-0 font-poppins text-[10px] text-white/35">
                  {m.createdAt.toLocaleDateString(undefined, {
                    month: "short",
                    day: "numeric",
                  })}
                </span>
              </div>

              <div className="mt-1.5 flex flex-wrap items-center gap-x-3 gap-y-1">
                <span className="font-poppins text-[11px] text-white/50">
                  {m.email}
                </span>
                <span className="font-poppins text-[11px] text-white/50">
                  {m.phoneNumber}
                </span>
              </div>

              {m.projectType && (
                <span className="mt-2 inline-flex items-center rounded-lg border border-white/10 bg-white/5 px-2 py-0.5 text-[9px] text-white/60">
                  {PROJECT_TYPE_LABELS[m.projectType] ?? m.projectType}
                </span>
              )}

              <p className="mt-2 font-poppins text-[11px] text-white/50">
                {truncateMessage(m.message)}
              </p>
            </div>
          </td>

          <td className="hidden px-4 py-4 font-poppins whitespace-nowrap text-white/80 md:table-cell lg:px-5">
            {m.name}
          </td>
          <td className="hidden px-4 py-4 font-poppins whitespace-nowrap md:table-cell lg:px-5">
            {m.email}
          </td>
          <td className="hidden px-4 py-4 font-poppins whitespace-nowrap md:table-cell lg:px-5">
            {m.phoneNumber}
          </td>
          <td className="hidden px-4 py-4 font-poppins whitespace-nowrap md:table-cell lg:px-5">
            {m.projectType ? (
              <span className="inline-flex items-center rounded-lg border border-white/10 bg-white/5 px-2.5 py-1 text-[10px] text-white/60 shadow-[inset_0_1px_0_rgba(255,255,255,0.04)] backdrop-blur-md">
                {PROJECT_TYPE_LABELS[m.projectType] ?? m.projectType}
              </span>
            ) : (
              <span className="text-white/25">—</span>
            )}
          </td>
          <td className="hidden max-w-xs px-4 py-4 font-poppins md:table-cell lg:px-5">
            <span className="text-white/60">{truncateMessage(m.message)}</span>
          </td>
          <td className="hidden whitespace-nowrap px-4 py-4 font-poppins text-white/35 md:table-cell lg:px-5">
            {m.createdAt.toLocaleDateString(undefined, {
              month: "short",
              day: "numeric",
              year: "numeric",
            })}{" "}
            {m.createdAt.toLocaleTimeString([], {
              hour: "2-digit",
              minute: "2-digit",
            })}
          </td>
        </tr>
      ))}

      {messages.length === 0 && (
        <tr>
          <td
            colSpan={6}
            className="px-5 py-16 text-center font-poppins text-xs text-white/30"
          >
            No messages found.
          </td>
        </tr>
      )}
    </tbody>
  );
}
