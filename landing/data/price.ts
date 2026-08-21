interface Price {
  for: string;
  title: string;
  price: number;
  priceLabel: string;
  shortDescription: string;
  description: string[];
  delivery: string;
  isFeatured?: boolean;
  category: "personal" | "business" | "revamp";
  pagesIncluded: string;
  revisions: string;
  supportPeriod: string;
  bestFor: string[];
}

export const PRICES: Price[] = [
  {
    for: "For Professionals",
    title: "Personal Website",
    price: 12000,
    priceLabel: "Starting from",
    category: "personal",
    delivery: "3–6 days",
    pagesIncluded: "Up to 2 pages",
    revisions: "2 rounds of revisions",
    supportPeriod: "1 weeks post-launch support",
    bestFor: ["Freelancers", "Portfolio sites", "Personal brands"],

    shortDescription:
      "A professional website to showcase who you are, what you do, and the work you're proud of.",

    description: [
      "A site that makes you look credible the moment someone lands on it",
      "Looks great on phone, tablet, and desktop — no awkward zooming",
      "Built to help people find you on Google",
      "Your own domain (yourname.com) and a matching professional email",
      "Contact & about sections that turn visitors into inquiries",
      "Loads fast — no one waits around for a slow site",
      "Secured with SSL so visitors (and Google) trust it",
      "See how many people visit and where they come from",
    ],
  },

  {
    for: "For Growing Businesses",
    title: "Company Website",
    price: 25000,
    priceLabel: "Starting from",
    category: "business",
    isFeatured: true,
    delivery: "3–4 weeks",
    pagesIncluded: "Up to 10 pages",
    revisions: "3 rounds of revisions",
    supportPeriod: "1 month post-launch support",
    bestFor: ["Small businesses", "Startups", "Service providers"],

    shortDescription:
      "A complete business website built to establish credibility, attract customers, and turn visitors into inquiries.",

    description: [
      "A website that makes new visitors trust you enough to reach out",
      "Capture leads directly from your site with built-in inquiry forms",
      "One-tap WhatsApp and call buttons — customers reach you instantly",
      "Show up when local customers search for what you offer",
      "Find you on Google Maps and know exactly where to visit",
      "Dedicated pages for your services and team, built to convert",
      "Looks sharp and works smoothly on any device",
      "Business email that matches your domain (you@yourcompany.com)",
      "Track visitors, inquiries, and what's actually working",
      "Fast-loading pages so customers don't bounce before they see you",
      "Secured with SSL so customers — and Google — trust your site",
    ],
  },

  {
    for: "For Existing Websites",
    title: "Website Revamp",
    price: 20000,
    priceLabel: "Starting from",
    category: "revamp",
    delivery: "2–4 weeks",
    pagesIncluded: "Existing pages",
    revisions: "2 rounds of revisions",
    supportPeriod: "3 weeks post-launch support",
    bestFor: [
      "Outdated websites",
      "Slow loading sites",
      "Poor mobile experience",
    ],

    shortDescription:
      "Transform an outdated website into a modern, faster, and more effective digital experience.",

    description: [
      "A full redesign that finally matches how good your business actually is",
      "Feels modern and works flawlessly on phones, tablets, and desktops",
      "Loads noticeably faster — no more losing visitors to a slow site",
      "Passes Google's speed & experience checks, which helps your ranking",
      "An SEO audit to fix what's quietly costing you search visibility",
      "Content reorganized so visitors find what they need immediately",
      "Forms that actually convert, not just sit there unused",
      "WhatsApp & Maps integration if your current site is missing it",
      "Analytics set up properly, so your data is finally accurate",
      "A full security review to close any gaps in your current setup",
      "Tested across real devices, not just previews",
      "Rebuilt on a modern, maintainable foundation — no more patchwork fixes",
    ],
  },
];
