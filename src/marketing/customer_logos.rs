use dioxus::prelude::*;

#[component]
pub fn Customers(class: Option<String>) -> Element {
    rsx! {
        section {
            class: format!("flex flex-col items-center {}", class.unwrap_or("".to_string())),
            span {
                class: "badge badge-primary badge-outline",
                "Trusted by the World's Best Companies"
            }
            h3 {
                class: "mt-4 mb-4",
                "Built with support from our partners"
            }
            div {
                class: "flex gap-6 space-between",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 120 60",
                    fill: "currentColor",
                    height: "40",
                    width: "auto",
                    path {
                        d: "M64.87 11.572l1.144.02 3.108-3.108.15-1.317c-2.47-2.197-5.72-3.534-9.277-3.534-6.44 0-11.876 4.382-13.486 10.318.34-.237 1.065-.06 1.065-.06l6.212-1.022s.32-.53.48-.497a7.76 7.76 0 0 1 10.605-.8z"
                    },
                    path {
                        d: "M73.5 13.962a13.99 13.99 0 0 0-4.216-6.796l-4.402 4.402a7.75 7.75 0 0 1 2.895 6.039v.777c2.142 0 3.88 1.743 3.88 3.88s-1.743 3.88-3.88 3.88h-7.762l-.777.78v4.658l.777.773h7.762A10.11 10.11 0 0 0 77.86 22.265c-.004-3.436-1.736-6.48-4.37-8.303z"
                    },
                    path {
                        d: "M52.234 32.362h7.76V26.15h-7.76a3.84 3.84 0 0 1-1.597-.347l-1.12.343-3.108 3.108-.272 1.05a9.96 9.96 0 0 0 6.098 2.06z"
                    },
                    path {
                        d: "M52.234 12.175A10.11 10.11 0 0 0 42.14 22.269a10.08 10.08 0 0 0 4 8.04l4.5-4.5a3.88 3.88 0 0 1-2.288-3.538c0-2.142 1.743-3.88 3.88-3.88a3.89 3.89 0 0 1 3.538 2.288l4.5-4.5c-1.846-2.43-4.76-4-8.04-4z"
                    },
                    path {
                        d: "M12 51.937c-2.12 0-3.94-.75-5.474-2.25s-2.3-3.304-2.3-5.408.765-3.908 2.3-5.408S9.883 36.62 12 36.62a7.32 7.32 0 0 1 5.249 2.11l-1.477 1.477a5.32 5.32 0 0 0-3.773-1.495c-1.53 0-2.83.54-3.896 1.627a5.41 5.41 0 0 0-1.597 3.941c0 1.546.53 2.857 1.597 3.94a5.25 5.25 0 0 0 3.896 1.627c1.558 0 2.845-.5 3.87-1.534.6-.6 1-1.5 1.14-2.635h-5.006v-2.092h7.044c.075.372.1.8.1 1.3 0 2.056-.603 3.686-1.813 4.895-1.372 1.435-3.15 2.15-5.345 2.15zm16.37-1.4c-.96.94-2.13 1.4-3.512 1.4s-2.554-.47-3.512-1.4-1.438-2.113-1.438-3.52.48-2.58 1.438-3.52 2.13-1.4 3.512-1.4 2.554.47 3.512 1.4 1.438 2.116 1.438 3.52-.48 2.58-1.438 3.52zm-5.474-1.38a2.63 2.63 0 0 0 1.963.849c.76 0 1.414-.282 1.963-.85s.822-1.28.822-2.14c0-.87-.27-1.588-.813-2.15s-1.198-.84-1.972-.84a2.64 2.64 0 0 0-1.972.84c-.543.56-.813 1.276-.813 2.15 0 .858.273 1.573.822 2.14zm16.273 1.38c-.96.94-2.13 1.4-3.512 1.4s-2.554-.47-3.512-1.4-1.438-2.113-1.438-3.52.48-2.58 1.438-3.52 2.13-1.4 3.512-1.4 2.554.47 3.512 1.4 1.438 2.116 1.438 3.52-.48 2.58-1.438 3.52zm-5.474-1.38a2.63 2.63 0 0 0 1.963.849c.76 0 1.414-.282 1.963-.85s.822-1.28.822-2.14c0-.87-.27-1.588-.813-2.15s-1.198-.84-1.972-.84a2.64 2.64 0 0 0-1.972.84c-.543.56-.813 1.276-.813 2.15 0 .858.273 1.573.822 2.14zm12.573 7.22c-1.095 0-2.017-.294-2.764-.88s-1.282-1.264-1.606-2.038l1.888-.783c.198.474.5.885.933 1.234s.94.522 1.552.522c.822 0 1.468-.25 1.933-.747s.7-1.216.7-2.15v-.7h-.075c-.6.747-1.477 1.122-2.596 1.122-1.258 0-2.36-.48-3.307-1.438a4.76 4.76 0 0 1-1.42-3.476 4.82 4.82 0 0 1 1.42-3.503c.945-.963 2.05-1.447 3.307-1.447.56 0 1.068.105 1.522.318s.813.474 1.074.783h.075V42.4h2.056v8.857c0 1.72-.438 3.004-1.318 3.86-.88.85-2.002 1.28-3.373 1.28zm.15-6.372a2.41 2.41 0 0 0 1.879-.849c.504-.567.756-1.273.756-2.122 0-.858-.252-1.576-.756-2.15a2.4 2.4 0 0 0-1.879-.858c-.76 0-1.408.288-1.942.858s-.804 1.288-.804 2.15c0 .846.267 1.555.804 2.122s1.183.85 1.942.85zM54.62 37.14v14.5h-2.167v-14.5zm5.94 14.796c-1.396 0-2.56-.474-3.494-1.42s-1.402-2.116-1.402-3.512c0-1.444.45-2.63 1.354-3.55a4.45 4.45 0 0 1 3.298-1.384c.597 0 1.153.108 1.663.327a3.92 3.92 0 0 1 1.27.84 5.84 5.84 0 0 1 .804.999 6.12 6.12 0 0 1 .486.972l.225.56L58.17 48.5c.5.996 1.3 1.495 2.392 1.495.996 0 1.807-.453 2.428-1.363l1.68 1.122c-.375.56-.903 1.065-1.588 1.513s-1.528.67-2.524.67zm-2.746-5.08l4.4-1.83c-.126-.312-.354-.564-.7-.756a2.26 2.26 0 0 0-1.14-.288c-.636 0-1.23.26-1.783.783s-.82 1.222-.795 2.092zm18.33 5.08c-1.97 0-3.62-.666-4.952-2s-2-2.995-2-4.988.666-3.656 2-4.988 2.983-2 4.952-2c2.017 0 3.656.73 4.913 2.185l-1.195 1.16c-.9-1.134-2.15-1.7-3.72-1.7-1.46 0-2.686.492-3.7 1.477s-1.504 2.272-1.504 3.866.5 2.884 1.504 3.87 2.233 1.477 3.7 1.477c1.606 0 2.977-.648 4.1-1.942l1.195 1.195a6.51 6.51 0 0 1-2.3 1.747 7.02 7.02 0 0 1-3.004.642zm8.556-.296h-1.72V38.263h1.72zm2.803-8.06c.885-.927 2-1.393 3.382-1.393s2.497.465 3.382 1.393 1.327 2.1 1.327 3.485-.44 2.557-1.327 3.485-2 1.393-3.382 1.393-2.497-.465-3.382-1.393-1.327-2.1-1.327-3.485.44-2.557 1.327-3.485zm1.28 5.883c.6.603 1.294.906 2.1.906s1.5-.303 2.1-.906.888-1.405.888-2.4-.297-1.798-.888-2.4-1.294-.906-2.1-.906-1.5.303-2.1.906-.888 1.405-.888 2.4.297 1.798.888 2.4zm16.32 2.177h-1.645v-1.27h-.075c-.26.435-.66.807-1.195 1.1s-1.1.46-1.7.46c-1.147 0-2.014-.348-2.605-1.047s-.888-1.633-.888-2.803v-5.606h1.72v5.324c0 1.708.753 2.56 2.26 2.56a2.1 2.1 0 0 0 1.738-.858 3.13 3.13 0 0 0 .672-1.981v-5.045h1.72v9.157zm5.828.297c-1.183 0-2.206-.468-3.064-1.402s-1.288-2.092-1.288-3.476.43-2.542 1.288-3.476 1.882-1.402 3.064-1.402c.696 0 1.324.15 1.88.447s.97.672 1.243 1.122h.075l-.075-1.27v-4.22h1.72v13.38h-1.645v-1.27h-.075c-.273.447-.687.822-1.243 1.122-.555.294-1.183.444-1.88.444zm.28-1.57a2.74 2.74 0 0 0 2.065-.897c.567-.597.85-1.402.85-2.4s-.282-1.813-.85-2.4a2.74 2.74 0 0 0-2.065-.897c-.798 0-1.483.303-2.056.906s-.858 1.405-.858 2.4.285 1.798.858 2.4a2.73 2.73 0 0 0 2.056.906z"
                    }
                }
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "35.188 31.512 351.46 258.785",
                    fill: "currentColor",
                    height: "40",
                    width: "auto",
                    path {
                        d: "M384.195 282.109c0 3.771-2.769 6.302-6.047 6.302v-.023c-3.371.023-6.089-2.508-6.089-6.278 0-3.769 2.718-6.293 6.089-6.293 3.279-.001 6.047 2.523 6.047 6.292zm2.453 0c0-5.176-4.02-8.18-8.5-8.18-4.511 0-8.531 3.004-8.531 8.18 0 5.172 4.021 8.188 8.531 8.188 4.48 0 8.5-3.016 8.5-8.188m-9.91.692h.91l2.109 3.703h2.315l-2.336-3.859c1.207-.086 2.2-.66 2.2-2.285 0-2.02-1.393-2.668-3.75-2.668h-3.411v8.812h1.961l.002-3.703m0-1.492v-2.121h1.364c.742 0 1.753.06 1.753.965 0 .984-.523 1.156-1.398 1.156h-1.719M329.406 237.027l10.598 28.992H318.48l10.926-28.992zm-11.35-11.289l-24.423 61.88h17.245l3.863-10.935h28.903l3.656 10.935h18.722l-24.605-61.888-23.361.008zm-49.033 61.903h17.497v-61.922l-17.5-.004.003 61.926zm-121.467-61.926l-14.598 49.078-13.984-49.074-18.879-.004 19.972 61.926h25.207l20.133-61.926h-17.851zm70.725 13.484h7.521c10.909 0 17.966 4.898 17.966 17.609 0 12.713-7.057 17.612-17.966 17.612h-7.521v-35.221zm-17.35-13.484v61.926h28.365c15.113 0 20.049-2.512 25.385-8.147 3.769-3.957 6.207-12.642 6.207-22.134 0-8.707-2.063-16.469-5.66-21.305-6.48-8.648-15.816-10.34-29.75-10.34h-24.547zm-165.743-.086v62.012h17.645v-47.086l13.672.004c4.527 0 7.754 1.129 9.934 3.457 2.765 2.945 3.894 7.699 3.894 16.396v27.229h17.098v-34.262c0-24.453-15.586-27.75-30.836-27.75H35.188zm137.583.086l.007 61.926h17.489v-61.926h-17.496z"
                    },
                    path {
                        d: "M82.211 102.414s22.504-33.203 67.437-36.638V53.73c-49.769 3.997-92.867 46.149-92.867 46.149s24.41 70.564 92.867 77.026v-12.804c-50.237-6.32-67.437-61.687-67.437-61.687zm67.437 36.223v11.727c-37.968-6.77-48.507-46.237-48.507-46.237s18.23-20.195 48.507-23.47v12.867c-.023 0-.039-.007-.058-.007-15.891-1.907-28.305 12.938-28.305 12.938s6.958 24.99 28.363 32.182m0-107.125V53.73c1.461-.112 2.922-.207 4.391-.257 56.582-1.907 93.449 46.406 93.449 46.406s-42.343 51.488-86.457 51.488c-4.043 0-7.828-.375-11.383-1.005v13.739a75.04 75.04 0 0 0 9.481.612c41.051 0 70.738-20.965 99.484-45.778 4.766 3.817 24.278 13.103 28.289 17.167-27.332 22.884-91.031 41.33-127.144 41.33-3.481 0-6.824-.211-10.11-.528v19.306H305.68V31.512H149.648zm0 49.144V65.777c1.446-.101 2.903-.179 4.391-.226 40.688-1.278 67.382 34.965 67.382 34.965s-28.832 40.042-59.746 40.042c-4.449 0-8.438-.715-12.028-1.922V93.523c15.84 1.914 19.028 8.911 28.551 24.786l21.181-17.859s-15.461-20.277-41.524-20.277c-2.834-.001-5.545.198-8.207.483"
                    }
                }
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "123.5 281.88608274 778.8 460.11391726",
                    fill: "currentColor",
                    height: "40",
                    width: "auto",
                    path {
                        d: "m344.4 449.1c0 9.4 1.1 17.1 2.8 22.7a141 141 0 0 0 8.2 18.5 10.6 10.6 0 0 1 1.8 5.8c0 2.6-1.5 5.2-4.9 7.7l-16 10.8a12.5 12.5 0 0 1 -6.7 2.3c-2.5 0-5.1-1.3-7.6-3.6a81.7 81.7 0 0 1 -9.2-12c-2.6-4.4-5.2-9.3-8-15.1q-29.9 35.3-75.1 35.3c-21.4 0-38.5-6.2-51-18.5s-18.9-28.6-18.9-49.1 7.6-39.4 23.2-52.7 36.3-19.9 62.6-19.9a199 199 0 0 1 27.1 2c9.4 1.3 19.1 3.3 29.3 5.6v-18.6c0-19.5-4.1-33-12-41s-21.9-11.7-41.6-11.7a116.2 116.2 0 0 0 -27.6 3.3 205.5 205.5 0 0 0 -27.6 8.7 68.7 68.7 0 0 1 -8.9 3.3 16.9 16.9 0 0 1 -4.1.8c-3.6 0-5.4-2.6-5.4-8v-12.5c0-4.1.5-7.2 1.8-8.9s3.6-3.6 7.2-5.4a142.4 142.4 0 0 1 32.1-11.5 152 152 0 0 1 39.9-4.9c30.4 0 52.6 6.9 66.9 20.7s21.2 34.8 21.2 63v82.9zm-103.7 38.9a81.7 81.7 0 0 0 26.3-4.7c9.2-3 17.4-8.7 24.3-16.3a40.7 40.7 0 0 0 8.7-16.4 94 94 0 0 0 2.5-22.3v-10.7a228.2 228.2 0 0 0 -23.5-4.4 195.7 195.7 0 0 0 -24-1.5c-17.1 0-29.6 3.3-38 10.2s-12.5 16.7-12.5 29.5 3 20.9 9.4 27.1 15.1 9.5 26.8 9.5zm205.1 27.6c-4.6 0-7.6-.8-9.7-2.6s-3.8-5.1-5.3-10l-60.1-197.7c-1.5-5.1-2.3-8.5-2.3-10.3 0-4 2.1-6.3 6.2-6.3h25c4.9 0 8.2.7 10 2.5s3.5 5.1 5.1 10l42.9 169.4 39.8-169.4c1.3-5.1 2.8-8.5 4.9-10s5.6-2.5 10.2-2.5h20.4c4.9 0 8.2.7 10.3 2.5s3.8 5.1 4.8 10l40.4 171.4 44.2-171.4c1.5-5.1 3.3-8.5 5.1-10s5.3-2.5 9.9-2.5h23.8c4.1 0 6.4 2 6.4 6.3a32.6 32.6 0 0 1 -.5 4.1 41.3 41.3 0 0 1 -1.8 6.4l-61.6 197.8c-1.5 5.1-3.3 8.4-5.4 10s-5.3 2.5-9.7 2.5h-21.9c-4.9 0-8.2-.7-10.2-2.5s-3.9-5.1-4.9-10.3l-39.6-165-39.3 164.8c-1.3 5.1-2.8 8.4-4.9 10.2s-5.6 2.6-10.2 2.6zm328.3 6.9a167.8 167.8 0 0 1 -39.4-4.6c-12.7-3.1-22.7-6.4-29.3-10.2-4.1-2.4-6.9-4.9-8-7.2a18.6 18.6 0 0 1 -1.5-7.2v-13c0-5.4 2-8 5.9-8a15.4 15.4 0 0 1 4.6.8c1.5.5 3.8 1.5 6.4 2.6a135.8 135.8 0 0 0 28.1 8.9 148.7 148.7 0 0 0 30.4 3.1c16 0 28.6-2.8 37.2-8.4s13.3-13.9 13.3-24.4a25.1 25.1 0 0 0 -6.9-17.9c-4.6-4.8-13.3-9.2-25.8-13.3l-37-11.5c-18.7-5.9-32.4-14.6-40.9-26.1a61 61 0 0 1 -12.7-37.1c0-10.7 2.3-20.2 6.9-28.4a64.3 64.3 0 0 1 18.3-20.9 82.2 82.2 0 0 1 26.6-13.4 113.8 113.8 0 0 1 32.2-4.3 140.1 140.1 0 0 1 17.1 1c5.9.8 11.2 1.8 16.6 2.8s10 2.6 14.6 4.1a64 64 0 0 1 10.7 4.6c3.6 2.1 6.1 4.1 7.7 6.4a14.3 14.3 0 0 1 2.3 8.5v12c0 5.4-2.1 8.2-5.9 8.2-2.1 0-5.4-1-9.7-3.1-14.6-6.6-30.9-10-49.1-10-14.5 0-26 2.3-33.9 7.2s-12 12.3-12 22.8a24 24 0 0 0 7.6 18.1c5.1 4.9 14.6 9.8 28.1 14.1l36.3 11.5c18.4 5.9 31.7 14.1 39.6 24.6a58 58 0 0 1 11.7 35.8 66.5 66.5 0 0 1 -6.6 29.7 68.5 68.5 0 0 1 -18.7 22.5c-7.9 6.4-17.3 11-28.3 14.3a120.7 120.7 0 0 1 -36.5 5.4z"
                    },
                    path {
                        d: "m822.3 646.8c-84 62.2-206.1 95.2-311.1 95.2-147.1 0-279.7-54.5-379.8-145.1-7.9-7.1-.8-16.8 8.7-11.2 108.3 62.9 241.9 101 380.1 101 93.2 0 195.6-19.4 289.9-59.3 14-6.4 26 9.2 12.2 19.4zm35-39.9c-10.7-13.8-71-6.6-98.3-3.3-8.2 1-9.5-6.1-2.1-11.5 48.1-33.8 127-24.1 136.2-12.8s-2.6 90.6-47.5 128.4c-6.9 5.9-13.6 2.8-10.5-4.8 10.2-25.4 33-82.4 22.2-96z"
                    }
                }
            }
        }
    }
}
