use crate::components::PageLayout;
use axum::response::{Html, IntoResponse};
use momenta::prelude::*;

struct Idea {
    number: u8,
    title: &'static str,
    description: &'static str,
    track: &'static str,
}

struct ArchivedProject {
    name: &'static str,
    description: &'static str,
    url: &'static str,
}

const IDEAS: &[Idea] = &[
    Idea {
        number: 2,
        title: "Memory Pool Allocator",
        description: "A fixed-size pool with safe allocation, deallocation, and error handling.",
        track: "Systems programming",
    },
    Idea {
        number: 3,
        title: "Reference Counting Smart Pointer",
        description: "A custom Rc type with cloning, cleanup, and strong reference counts.",
        track: "Systems programming",
    },
    Idea {
        number: 4,
        title: "Thread-Safe Counter",
        description: "A shared counter exercised concurrently through Arc and Mutex.",
        track: "Systems programming",
    },
    Idea {
        number: 5,
        title: "Lock-Free Stack",
        description: "A concurrent stack built with atomic push and pop operations.",
        track: "Systems programming",
    },
    Idea {
        number: 6,
        title: "Custom Async Runtime",
        description: "A single-threaded executor with spawn and block_on support.",
        track: "Systems programming",
    },
    Idea {
        number: 7,
        title: "Memory Mapped File Reader",
        description: "An efficient reader for seeking through chunks of large mapped files.",
        track: "Systems programming",
    },
    Idea {
        number: 8,
        title: "Intrusive Linked List",
        description: "A safe doubly linked list whose nodes own their linking pointers.",
        track: "Systems programming",
    },
    Idea {
        number: 9,
        title: "Work-Stealing Queue",
        description: "A concurrent deque with local push and pop plus cross-thread stealing.",
        track: "Systems programming",
    },
    Idea {
        number: 10,
        title: "Copy-on-Write String",
        description: "A string that shares storage until mutation, targeting Rust 1.60 to 1.70.",
        track: "Systems programming",
    },
    Idea {
        number: 11,
        title: "B-Tree Implementation",
        description: "A configurable B-tree with insertion, deletion, search, split, and merge.",
        track: "Systems programming",
    },
    Idea {
        number: 12,
        title: "Virtual Memory Simulator",
        description: "A page-table and TLB simulator with LRU and FIFO replacement.",
        track: "Systems programming",
    },
    Idea {
        number: 13,
        title: "Custom Garbage Collector",
        description: "A mark-and-sweep collector with cycle detection and weak references.",
        track: "Systems programming",
    },
    Idea {
        number: 14,
        title: "Kernel Module Interface",
        description: "A safe Rust wrapper for basic Linux kernel device-driver APIs.",
        track: "Systems programming",
    },
    Idea {
        number: 15,
        title: "HTTP Parser",
        description: "An HTTP/1.1 request parser with robust malformed-input handling.",
        track: "Web development",
    },
    Idea {
        number: 16,
        title: "Static File Server",
        description: "An HTTP server with MIME detection and correct response status codes.",
        track: "Web development",
    },
    Idea {
        number: 17,
        title: "JSON API Server",
        description: "An in-memory CRUD API with structured JSON requests and responses.",
        track: "Web development",
    },
    Idea {
        number: 18,
        title: "Database Connection Pool",
        description: "A bounded connection pool with health checks and reconnection.",
        track: "Web development",
    },
    Idea {
        number: 19,
        title: "WebSocket Chat Server",
        description: "A multi-client chat server with broadcasting and user management.",
        track: "Web development",
    },
    Idea {
        number: 20,
        title: "Rate Limiting Middleware",
        description: "Per-IP and per-user limits using token buckets or sliding windows.",
        track: "Web development",
    },
    Idea {
        number: 21,
        title: "OAuth2 Client",
        description: "An authorization-code client with token refresh and error handling.",
        track: "Web development",
    },
    Idea {
        number: 22,
        title: "GraphQL Server",
        description: "A schema, parser, and executor supporting nested queries and mutations.",
        track: "Web development",
    },
    Idea {
        number: 23,
        title: "Microservice Communication",
        description: "A service-mesh layer with discovery, balancing, and circuit breakers.",
        track: "Web development",
    },
    Idea {
        number: 24,
        title: "Real-time Analytics Engine",
        description: "Streaming analytics with windowed aggregations and alerts.",
        track: "Web development",
    },
    Idea {
        number: 25,
        title: "API Gateway",
        description: "A framework-free gateway for routing, auth, limits, and transformations.",
        track: "Web development",
    },
    Idea {
        number: 26,
        title: "Distributed Cache",
        description: "A replicated cache with consistent hashing and Redis protocol support.",
        track: "Web development",
    },
    Idea {
        number: 27,
        title: "Event Sourcing Framework",
        description: "An event store with commands, projections, rebuilds, and snapshots.",
        track: "Web development",
    },
    Idea {
        number: 28,
        title: "Multi-tenant SaaS Platform",
        description: "An isolated tenant platform with quotas, configuration, billing, and usage.",
        track: "Web development",
    },
    Idea {
        number: 29,
        title: "File Diff Tool",
        description: "A colorized command-line comparison of line-by-line file changes.",
        track: "CLI and tools",
    },
    Idea {
        number: 30,
        title: "Log Parser",
        description: "A parser for common server logs with date and status filtering.",
        track: "CLI and tools",
    },
    Idea {
        number: 31,
        title: "Directory Synchronizer",
        description: "Two-way directory updates with dry runs and progress reporting.",
        track: "CLI and tools",
    },
    Idea {
        number: 32,
        title: "System Monitor",
        description: "Live and historical CPU, memory, disk, and network monitoring.",
        track: "CLI and tools",
    },
    Idea {
        number: 33,
        title: "Regex Engine",
        description: "A matcher for common regular-expression operators and search patterns.",
        track: "CLI and tools",
    },
    Idea {
        number: 34,
        title: "Cargo install clone",
        description: "A small package manager with version constraints and dependency resolution.",
        track: "CLI and tools",
    },
    Idea {
        number: 35,
        title: "Cargo fmt clone",
        description: "A formatter with configurable indentation and maximum line width.",
        track: "CLI and tools",
    },
    Idea {
        number: 36,
        title: "Backup Tool",
        description: "Incremental, compressed, encrypted backups with restore and integrity checks.",
        track: "CLI and tools",
    },
    Idea {
        number: 37,
        title: "Database Migration Tool",
        description: "Ordered schema migrations with rollback and history tracking.",
        track: "CLI and tools",
    },
    Idea {
        number: 38,
        title: "Static Site Generator",
        description: "Markdown, front matter, templates, asset processing, and live reload.",
        track: "CLI and tools",
    },
    Idea {
        number: 39,
        title: "Fuzzing Framework",
        description: "Random and mutated input generation with crash and coverage tracking.",
        track: "CLI and tools",
    },
    Idea {
        number: 40,
        title: "Compiler Frontend",
        description: "Lexing, parsing, AST generation, semantic checks, and useful errors.",
        track: "CLI and tools",
    },
    Idea {
        number: 41,
        title: "Build System",
        description: "Parallel and incremental dependency builds with caching and artifacts.",
        track: "CLI and tools",
    },
];

const ARCHIVED_PROJECTS: &[ArchivedProject] = &[
    ArchivedProject {
        name: "chakra-svelte",
        description: "An early Svelte component-library repository.",
        url: "https://github.com/elcharitas/chakra-svelte",
    },
    ArchivedProject {
        name: "chakra-ui-svelte",
        description: "Modular, accessible Chakra-inspired components for Svelte.",
        url: "https://github.com/elcharitas/chakra-ui-svelte",
    },
    ArchivedProject {
        name: "custom-printf-c",
        description: "A printf implementation written in C.",
        url: "https://github.com/elcharitas/custom-printf-c",
    },
    ArchivedProject {
        name: "elcharitas-preact",
        description: "An earlier Preact version of this blog and portfolio.",
        url: "https://github.com/elcharitas/elcharitas-preact",
    },
    ArchivedProject {
        name: "nowpayment-react",
        description: "A sample NOWPayments integration for React.",
        url: "https://github.com/elcharitas/nowpayment-react",
    },
    ArchivedProject {
        name: "paymematic-polygon-buidl",
        description: "Personal payment links for accepting MATIC.",
        url: "https://github.com/elcharitas/paymematic-polygon-buidl",
    },
    ArchivedProject {
        name: "polygon-screening-task",
        description: "A preserved Polygon engineering screening exercise.",
        url: "https://github.com/elcharitas/polygon-screening-task",
    },
    ArchivedProject {
        name: "token-manager-dashboard",
        description: "A simple dashboard for managing ERC-20 tokens.",
        url: "https://github.com/elcharitas/token-manager-dashboard",
    },
    ArchivedProject {
        name: "w3g-draft",
        description: "Early drafts for the Web3Gateway idea.",
        url: "https://github.com/elcharitas/w3g-draft",
    },
];

pub async fn graveyard_handler() -> impl IntoResponse {
    Html(GraveyardPage::render(&momenta::nodes::DefaultProps).to_string())
}

#[component]
pub fn GraveyardPage() -> Node {
    let tracks = ["Systems programming", "Web development", "CLI and tools"];

    rsx! {
        <PageLayout title="Idea Graveyard">
            <section class="py-4 md:py-8 space-y-4">
                <h1 class="text-4xl md:text-5xl font-semibold text-white">"Idea graveyard"</h1>
                <div class="section-rule"></div>
                <p class="text-base md:text-lg text-zinc-300 leading-relaxed max-w-3xl">
                    "A record of ideas waiting for their turn and original projects I have formally archived. Nothing else is presumed dead."
                </p>
            </section>

            <section class="space-y-8">
                <div class="space-y-2">
                    <p class="text-xs uppercase tracking-[0.1em]" style="color: var(--accent);">"Open ideas · 40"</p>
                    <h2 class="text-2xl md:text-3xl font-semibold text-white">"Rusty June backlog"</h2>
                    <p class="text-sm text-zinc-400 max-w-3xl">
                        "Forty concrete Rust build ideas, preserved as open issues in "
                        <a href="https://github.com/elcharitas/rusty-june/issues/1" target="_blank" rel="noopener noreferrer" class="text-zinc-200 hover:text-white underline underline-offset-4">"rusty-june"</a>
                        "."
                    </p>
                </div>

                {tracks.iter().map(|track| {
                    <div class="space-y-4">
                        <h3 class="text-lg font-semibold text-zinc-100">{track}</h3>
                        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
                            {IDEAS.iter().filter(|idea| idea.track == *track).map(|idea| {
                                let url = format!("https://github.com/elcharitas/rusty-june/issues/{}", idea.number);
                                <article class="card-item rounded-2xl p-5 space-y-3 soft-lift">
                                    <div class="flex items-start justify-between gap-3">
                                        <h4 class="text-lg font-semibold text-zinc-100 leading-snug">{idea.title}</h4>
                                        <a href={url.as_str()} target="_blank" rel="noopener noreferrer" class="social-link shrink-0" aria_label={format!("Open issue {}", idea.number)}>
                                            <span class="text-xs">{format!("#{}", idea.number)}</span>
                                        </a>
                                    </div>
                                    <p class="text-sm text-zinc-400 leading-relaxed">{idea.description}</p>
                                </article>
                            })}
                        </div>
                    </div>
                })}
            </section>

            <section class="space-y-5 pb-12">
                <div class="space-y-2">
                    <p class="text-xs uppercase tracking-[0.1em]" style="color: var(--accent);">"Archived originals · 9"</p>
                    <h2 class="text-2xl md:text-3xl font-semibold text-white">"Projects laid to rest"</h2>
                    <p class="text-sm text-zinc-400 max-w-3xl">"These nine original repositories are explicitly marked archived on GitHub."</p>
                </div>
                <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
                    {ARCHIVED_PROJECTS.iter().map(|project| {
                        <article class="card-item rounded-2xl p-5 space-y-3 soft-lift">
                            <div class="flex items-start justify-between gap-3">
                                <h3 class="text-lg font-semibold text-zinc-100 leading-snug">{project.name}</h3>
                                <a href={project.url} target="_blank" rel="noopener noreferrer" class="social-link shrink-0" aria_label={format!("Open {} on GitHub", project.name)}>
                                    <i class="fas fa-arrow-up-right-from-square text-sm"></i>
                                </a>
                            </div>
                            <p class="text-sm text-zinc-400 leading-relaxed">{project.description}</p>
                        </article>
                    })}
                </div>
            </section>
        </PageLayout>
    }
}
