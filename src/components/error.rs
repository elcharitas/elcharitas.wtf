use simple_rsx::*;

#[component]
pub fn Error() -> Node {
    rsx! {
      <div class="min-h-screen flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
              <div class="text-center">
                <svg
                    class=" mx-auto h-12 w-auto text-zinc-500"
                    fill="none"
                    height="24"
                    stroke="currentColor"
                    stroke_linecap="round"
                    stroke_linejoin="round"
                    stroke_width="2"
                    viewBox="0 0 24 24"
                    width="24"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <circle cx="12" cy="12" r="10"></circle>
                    <line x1="12" x2="12" y1="8" y2="12"></line>
                    <line x1="12" x2="12.01" y1="16" y2="16"></line>
                </svg>
                <h2 class="mt-6 text-3xl font-extrabold text-zinc-100">
                  Oops! Something went wrong
                </h2>
                <p class="mt-2 text-sm text-zinc-300">
                  This file, page or resource could not be found
                </p>
              </div>
              <div class="mt-8 space-y-6 flex justify-center">
                <a
                  class="group relative w-1/2 flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-zinc-700 hover:bg-zinc-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-zinc-600"
                  href="/"
                >
                  Go Home
                </a>
              </div>
            </div>
        </div>
    }
}
