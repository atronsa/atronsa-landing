interface Project {
  title: string;
  category: string;
  className: string;
  image: string;
  link: string;
}

export const PROJECTS: Project[] = [
  {
    title: "Sahwa Coffee",
    category: "Ethiopian Coffee Export Company",
    className: "md:col-span-1",
    image: "/images/sahwa-coffee.webp",
    link: "http://sahwacoffee.com/",
  },
  {
    title: "Help Ethiopia",
    category: "Non-Profit Medical Organization",
    className: "md:col-span-1",
    image: "/images/help-ethiopia.webp",
    link: "https://app.helpethiopia.org.et/",
  },
  {
    title: "Dagim's Portfolio",
    category: "Professional Portfolio Website",
    className: "md:col-span-1",
    image: "/images/dagim.webp",
    link: "https://dagim.dev/",
  },
];
