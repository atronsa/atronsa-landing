import Link from "next/link";
import { auth, signOut } from "@/lib/auth";
import { redirect, notFound } from "next/navigation";
import { prisma } from "@/lib/prisma";

export const dynamic = "force-dynamic";

const PROJECT_TYPE_LABELS: Record<string, string> = {
  personal: "Personal Website",
  business: "Company Website",
  revamp: "Website Revamp",
  other: "Other",
};

export default async function MessageDetailPage({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  const session = await auth();
  if (!session) redirect("/admin/login");

  const { id } = await params;
  const message = await prisma.contactMessage.findUnique({
    where: { id: Number(id) },
  });

  if (!message) notFound();

  return (
    <main className="relative min-h-screen overflow-hidden bg-black px-4 sm:px-6 lg:px-16 pt-24 sm:pt-28 lg:pt-32">
      <div
        className="pointer-events-none absolute inset-0 opacity-30"
        style={{
          backgroundImage: `
            linear-gradient(rgba(255,255,255,0.025) 1px, transparent 1px),
            linear-gradient(90deg, rgba(255,255,255,0.025) 1px, transparent 1px)
          `,
          backgroundSize: "80px 80px",
          maskImage:
            "linear-gradient(to bottom, transparent, black 15%, black 85%, transparent)",
          WebkitMaskImage:
            "linear-gradient(to bottom, transparent, black 15%, black 85%, transparent)",
        }}
      />

      <div className="relative z-10 mx-auto max-w-3xl">
        <div className="flex flex-col gap-4 border-b border-white/[0.07] pb-6 sm:flex-row sm:items-center sm:justify-between">
          <div className="min-w-0">
            <h1 className="font-title text-2xl font-medium leading-[0.9] tracking-[-0.055em] text-white sm:text-3xl">
              Message Detail
            </h1>
          </div>

          <Link
            href="/admin"
            className="mb-3 mt-3 inline-flex items-center gap-2 rounded-xl border border-white/10 bg-white/4 px-3.5 py-2 font-poppins text-[11px] text-white/60 transition-all duration-300 hover:border-white/20 hover:bg-white/8 hover:text-white"
          >
            <svg
              className="h-3.5 w-3.5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1.5}
                d="M19 12H5m6 6l-6-6 6-6"
              />
            </svg>
            Back To Form
          </Link>
        </div>

        <div className="mt-6 overflow-hidden rounded-xl border border-white/8 bg-white/2.5 shadow-[inset_0_1px_0_rgba(255,255,255,0.07)] backdrop-blur-2xl">
          <div className="border-b border-white/8 bg-white/2 px-6 py-5">
            <div className="flex items-center justify-between">
              <h2 className="font-title text-xl text-white sm:text-2xl">
                {message.name}
              </h2>

              {message.projectType && (
                <span className="inline-flex items-center rounded-xl border border-white/10 bg-white/5 px-3 py-1.5 text-[10px] text-white/60 shadow-[inset_0_1px_0_rgba(255,255,255,0.04)] backdrop-blur-md">
                  {PROJECT_TYPE_LABELS[message.projectType] ??
                    message.projectType}
                </span>
              )}
            </div>

            <p className="mt-2 font-poppins text-[11px] text-white/35">
              {message.createdAt.toLocaleDateString(undefined, {
                weekday: "long",
                month: "long",
                day: "numeric",
                year: "numeric",
              })}{" "}
              at{" "}
              {message.createdAt.toLocaleTimeString([], {
                hour: "2-digit",
                minute: "2-digit",
              })}
            </p>
          </div>

          <div className="grid grid-cols-1 gap-4 px-6 py-6 sm:grid-cols-2">
            <div className="rounded-xl border border-white/[0.07] bg-white/2.5 p-4">
              <p className="font-poppins text-[10px] uppercase tracking-[0.12em] text-white/35">
                Email
              </p>
              <p className="mt-2 block font-poppins text-xs sm:text-sm text-white/80">
                {message.email}
              </p>
            </div>

            <div className="rounded-xl border border-white/[0.07] bg-white/2.5 p-4">
              <p className="font-poppins text-[10px] uppercase tracking-[0.12em] text-white/35">
                Phone
              </p>
              <p className="mt-2 block font-poppins text-xs sm:text-sm text-white/80">
                {message.phoneNumber}
              </p>
            </div>
          </div>

          <div className="border-t border-white/[0.07] px-6 py-6">
            <p className="font-poppins text-[10px] uppercase tracking-[0.12em] text-white/35">
              Message
            </p>
            <p className="mt-3 font-poppins text-xs sm:text-sm leading-relaxed text-white/70">
              {message.message}
            </p>
          </div>

          <div className="flex flex-wrap gap-2 border-t border-white/[0.07] px-6 py-5">
            <a
              href={`mailto:${message.email}`}
              className="inline-flex items-center gap-2 rounded-xl border border-white/10 bg-white/4 px-4 py-2.5 font-poppins text-[11px] text-white/60 transition-all duration-300 hover:border-white/20 hover:bg-white/8 hover:text-white"
            >
              <svg
                className="h-3.5 w-3.5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={1.5}
                  d="M21.75 6.75v10.5a2.25 2.25 0 01-2.25 2.25h-15a2.25 2.25 0 01-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0019.5 4.5h-15a2.25 2.25 0 00-2.25 2.25m19.5 0v.243a2.25 2.25 0 01-1.07 1.916l-7.5 4.615a2.25 2.25 0 01-2.36 0L3.32 8.91a2.25 2.25 0 01-1.07-1.916V6.75"
                />
              </svg>
              Reply via Email
            </a>

            <a
              href={`tel:${message.phoneNumber}`}
              className="inline-flex items-center gap-2 rounded-xl border border-white/10 bg-white/4 px-4 py-2.5 font-poppins text-[11px] text-white/60 transition-all duration-300 hover:border-white/20 hover:bg-white/8 hover:text-white"
            >
              <svg
                className="h-3.5 w-3.5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={1.5}
                  d="M2.25 6.75c0 8.284 6.716 15 15 15h2.25a2.25 2.25 0 002.25-2.25v-1.372c0-.516-.351-.966-.852-1.09l-4.423-1.106a1.125 1.125 0 00-1.173.417l-.97 1.293c-.5.667-1.45.88-2.22.502a12.035 12.035 0 01-6.657-6.657c-.378-.77-.165-1.72.502-2.22l1.293-.97c.277-.208.39-.553.317-.89L6.51 3.884A1.125 1.125 0 005.42 3.032H4.5A2.25 2.25 0 002.25 5.282v1.468z"
                />
              </svg>
              Call
            </a>

            <a
              href={`https://wa.me/${message.phoneNumber.replace(/[^0-9]/g, "")}`}
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 rounded-xl border border-white/10 bg-white/4 px-4 py-2.5 font-poppins text-[11px] text-white/60 transition-all duration-300 hover:border-white/20 hover:bg-white/8 hover:text-white"
            >
              <svg
                className="h-3.5 w-3.5"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 01-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 01-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 012.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0012.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 005.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 00-3.48-8.413Z" />
              </svg>
              WhatsApp
            </a>
          </div>
        </div>
      </div>
    </main>
  );
}
