use html_elements::HtmxAttributes;
use lazy_static::lazy_static;

use crate::{
    components::{AppLayout, LayoutProps},
    derive_component,
};
use crate::{jsx, shared::*};

struct NavigationInfo {
    name: &'static str,
    href: &'static str,
}

lazy_static! {
    static ref NAVIGATION: Vec<NavigationInfo> = vec![
        NavigationInfo {
            name: "🛠️ Projects",
            href: "/projects"
        },
        NavigationInfo {
            name: "✍🏼 Writings",
            href: "/writings"
        },
        NavigationInfo {
            name: "📄 Resume",
            href: "/resume"
        },
        NavigationInfo {
            name: "🌍 Adventures",
            href: "/adventures"
        },
    ];
}

derive_component! {
    pub HomePage {
        let article = jsx! {
            <input type="text" />
        };
        AppLayout::with(LayoutProps {
            title: "Home".to_string(),
            children: Rsx(hypertext::rsx! {
                <div class="flex flex-col items-center justify-center w-screen h-screen overflow-hidden bg-gradient-to-tl from-black via-zinc-600/20 to-black">
                    <nav class="my-16 animate-fade-in">
                        <div class="flex items-center justify-center gap-4">
                            {NAVIGATION.iter().zip(1..).map(|(item, _i)| hypertext::rsx_move! {
                                <a
                                    href=item.href
                                    class="text-sm duration-500 text-zinc-500 hover:text-zinc-300"
                                    hx_boost="true"
                                >
                                    {item.name}
                                </a>
                            }).render_all()}
                        </div>
                    </nav>
                    <h1 class="z-10 text-4xl text-zinc-100 duration-1000 cursor-default text-edge-outline animate-title font-display sm:text-6xl md:text-9xl whitespace-nowrap">
                        "elch🔥rit🔥s"
                    </h1>
                    <div class="my-16 mx-4 text-center animate-fade-in">
                        <h2 class="text-sm text-zinc-500 max-sm:w-[350px]">
                        Hi, I{"'"}m Jonathan, a proud believer, lover of music, philosophy, theology and technology.
                        <br />I{"'"}m a Software Engineer at{" "}
                        <a
                            target="_blank"
                            href="https://alphaday.com"
                            class="underline duration-500 hover:text-zinc-300"
                        >
                            Alphaday
                        </a> {", "}
                        and maintainer of the next generation framework - {" "}
                        <a
                            target="_blank"
                            href="https://ngyn.rs"
                            class="underline duration-500 hover:text-zinc-300"
                        >
                            ngyn
                        </a>{" "}
                        at night. "😇"
                        </h2>
                        <div class="pt-8">
                        <a
                            href="/connect"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="px-4 py-2 text-sm text-white bg-zinc-700 border border-zinc-700 rounded hover:bg-zinc-600 duration-500"
                            title="Let's get your questions answered"
                        >
                            Connect with me "🤝"
                        </a>
                        </div>
                    </div>
                    <div class="flex justify-center space-x-4 my-4">
                        <a
                            href="https://github.com/elcharitas"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-zinc-500 hover:text-zinc-300 duration-500"
                            title="I collaborate on github often"
                            aria-label="Follow me on github"
                        >
                        <svg class="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewbox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"></path><path d="M9 18c-4.51 2-5-2-7-2"></path></svg>
                        </a>
                        <a
                            href="https://linkedin.com/in/elcharitas"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-zinc-500 hover:text-zinc-300 duration-500"
                            aria-label="Connect with me on linkedin"
                        >
                        <svg class="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewbox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"></path><rect width="4" height="12" x="2" y="9"></rect><circle cx="4" cy="4" r="2"></circle></svg>
                        </a>
                        <a
                            href="https://instagram.com/iamelcharitas"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-zinc-500 hover:text-zinc-300 duration-500"
                            aria-label="Connect with me on instagram"
                        >
                        <svg class="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewbox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="20" x="2" y="2" rx="5" ry="5"></rect><path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z"></path><line x1="17.5" x2="17.51" y1="6.5" y2="6.5"></line></svg>
                        </a>
                        <a
                            href="https://twitter.com/iamelcharitas"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="text-zinc-500 hover:text-zinc-300 duration-500"
                            aria-label="Follow me on twitter"
                        >
                       <svg class="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewbox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 4s-.7 2.1-2 3.4c1.6 10-9.4 17.3-18 11.6 2.2.1 4.4-.6 6-2C3 15.5.5 9.6 3 5c2.2 2.6 5.6 4.1 9 4-.9-4.2 4-6.6 7-3.8 1.1 0 3-1.2 3-1.2z"/></svg>
                        </a>
                    </div>
                </div>
            })
        })
    }
}
