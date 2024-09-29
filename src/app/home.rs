use lazy_static::lazy_static;

use crate::shared::*;
use crate::{
    components::{AppLayout, LayoutProps},
    derive_component, rsx_move,
};

struct NavigationInfo {
    name: &'static str,
    href: &'static str,
}

lazy_static! {
    static ref NAVIGATION: Vec<NavigationInfo> = vec![
        NavigationInfo {
            name: "Home",
            href: "/"
        },
        NavigationInfo {
            name: "Projects",
            href: "/projects"
        },
        NavigationInfo {
            name: "Writings",
            href: "/writings"
        },
        NavigationInfo {
            name: "Resume",
            href: "/mod/resume"
        },
        NavigationInfo {
            name: "Adventures",
            href: "/adventures"
        },
    ];
}

derive_component! {
    pub Home {
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
                    <h2 class="text-sm text-zinc-500 max-sm:w-[300px]">
                    Hi, I{"'"}m Jonathan, a proud believer, lover of music, philosophy, theology and technology. I{"'"}m a Software Engineer at{" "}
                    <a
                        target="_blank"
                        href="https://alphaday.com"
                        class="underline duration-500 hover:text-zinc-300"
                    >
                        Alphaday
                    </a>
                    <br />
                    publicly building the next generation framework - {" "}
                    <a
                        target="_blank"
                        href="https://ngyn.rs"
                        class="underline duration-500 hover:text-zinc-300"
                    >
                        Ngyn
                    </a>{" "}
                    at night. "😇"
                    </h2>
                    <div class="pt-8">
                    <a
                        href="/mods/connect"
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
                    <FontAwesomeIcon
                        icon="github"
                        size=24
                        class="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110"
                    />
                    </a>
                    <a
                    href="https://linkedin.com/in/elcharitas"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-zinc-500 hover:text-zinc-300 duration-500"
                    aria-label="Connect with me on linkedin"
                    >
                    <FontAwesomeIcon
                        icon="linkedin"
                        size=24
                        class="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110"
                    />
                    </a>
                    <a
                    href="https://instagram.com/iamelcharitas"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-zinc-500 hover:text-zinc-300 duration-500"
                    aria-label="Connect with me on instagram"
                    >
                    <FontAwesomeIcon
                        icon="instagram"
                        size=24
                        class="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110"
                    />
                    </a>
                    <a
                    href="https://twitter.com/iamelcharitas"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-zinc-500 hover:text-zinc-300 duration-500"
                    aria-label="Follow me on twitter"
                    >
                    <FontAwesomeIcon
                        icon="twitter"
                        size=24
                        class="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110"
                    />
                    </a>
                    <a
                    href="https://links.dev/elcharitas"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="text-zinc-500 hover:text-zinc-300 duration-500"
                    aria-label="My links"
                    >
                    <FontAwesomeIcon
                        icon="link"
                        size=24
                        class="transition duration-500 ease-in-out transform hover:-translate-y-1 hover:scale-110"
                    />
                    </a>
                </div>
                </div>
            })
        })
    }
}
