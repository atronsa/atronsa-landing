import { ReactNode } from "react";

interface Service {
  className: string;
  title: string;
  description: string;
  icon: ReactNode;
  tags: string[];
  number: string;
}

export const SERVICES: Service[] = [
  {
    className: "lg:col-span-2",
    title: "Web Development & Design",
    description:
      "Custom websites and web applications crafted with precision from landing pages to complex platforms.",
    icon: (
      <svg
        className="h-4 w-4 sm:h-5 sm:w-5"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={1.5}
          d="M17.25 6.75L22.5 12l-5.25 5.25m-10.5 0L1.5 12l5.25-5.25m7.5-3l-4.5 16.5"
        />
      </svg>
    ),
    tags: ["Responsive Design", "E-commerce", "Web Apps", "UI/UX"],
    number: "01",
  },
  {
    className: "lg:col-span-1",
    title: "SEO Development",
    description:
      "Drive organic growth with technical SEO and performance optimization.",
    icon: (
      <svg
        className="h-4 w-4 sm:h-5 sm:w-5"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={1.5}
          d="M2.25 18L9 11.25l4.306 4.307a11.95 11.95 0 015.814-5.519l2.74-1.22m0 0l-5.94-2.28m5.94 2.28l-2.28 5.941"
        />
      </svg>
    ),
    tags: ["Technical SEO", "Performance", "Analytics"],
    number: "02",
  },
  {
    className: "lg:col-span-1",
    title: "Custom Software",
    description:
      "Tailored systems built to solve your unique business challenges.",
    icon: (
      <svg
        className="h-4 w-4 sm:h-5 sm:w-5"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={1.5}
          d="M11.42 15.17L17.25 21A2.652 2.652 0 0021 17.25l-5.877-5.877M11.42 15.17l2.496-3.03c.317-.384.74-.626 1.208-.766M11.42 15.17l-4.655 5.653a2.548 2.548 0 11-3.586-3.586l6.837-5.63m5.108-.233c.55-.164 1.163-.188 1.743-.14a4.5 4.5 0 004.486-6.336l-3.276 3.277a3.004 3.004 0 01-2.25-2.25l3.276-3.276a4.5 4.5 0 00-6.336 4.486c.091 1.076-.071 2.264-.904 2.95l-.102.085"
        />
      </svg>
    ),
    tags: ["CRM", "Automation", "API Integration"],
    number: "03",
  },
  {
    className: "lg:col-span-2",
    title: "Training & Support",
    description:
      "Empower your team with the skills and confidence to manage your digital presence.",
    icon: (
      <svg
        className="h-4 w-4 sm:h-5 sm:w-5"
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
      >
        <path
          strokeLinecap="round"
          strokeLinejoin="round"
          strokeWidth={1.5}
          d="M18 18.72a9.094 9.094 0 003.741-.479 3 3 0 00-4.682-2.72m.94 3.198l.001.031c0 .225-.012.447-.037.666A11.944 11.944 0 0112 21c-2.17 0-4.207-.576-5.963-1.584A6.062 6.062 0 016 18.719m12 0a5.971 5.971 0 00-.941-3.197m0 0A5.995 5.995 0 0012 12.75a5.995 5.995 0 00-5.058 2.772m0 0a3 3 0 00-4.681 2.72 8.986 8.986 0 003.74.477m.94-3.197a5.971 5.971 0 00-.94 3.197M15 6.75a3 3 0 11-6 0 3 3 0 016 0zm6 3a2.25 2.25 0 11-4.5 0 2.25 2.25 0 014.5 0zm-13.5 0a2.25 2.25 0 11-4.5 0 2.25 2.25 0 014.5 0z"
        />
      </svg>
    ),
    tags: ["Team Training", "Documentation", "Ongoing Support"],
    number: "04",
  },
];
