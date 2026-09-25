use leptos::prelude::*;

#[component]
pub fn PreviewContent() -> impl IntoView {
    view! {
        <div class="content preview">
            <p>
                "Lorem "<span class="color-1">"ipsum"</span>
                " dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor "
                <span class="color-2">"incididunt"</span>
                " ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud "
                <span class="color-3">"exercitation"</span>
                " ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor
                in "<span class="color-4">"reprehenderit"</span>" in voluptate velit esse cillum
                dolore eu fugiat nulla pariatur. "<span class="color-5">"Excepteur"</span>
                " sint occaecat cupidatat non proident, sunt in culpa qui "
                <span class="color-6">"officia deserunt"</span>" mollit anim id est laborum."
            </p>
            <p>
                "Here is a " <a href="https://www.lipsum.com/feed/html">"hyperlink to somewhere"</a>
                "."

            </p>
            <table>
                <tbody>
                    <tr>
                        <td class="red">Red</td>
                        <td class="green">Green</td>
                        <td class="blue">Blue</td>
                        <td class="cyan">Cyan</td>
                        <td class="purple">Purple</td>
                        <td class="yellow">Yellow</td>
                    </tr>
                    <tr>
                        <td class="red-background">Red</td>
                        <td class="green-background">Green</td>
                        <td class="blue-background">Blue</td>
                        <td class="cyan-background">Cyan</td>
                        <td class="purple-background">Purple</td>
                        <td class="yellow-background">Yellow</td>
                    </tr>
                </tbody>
            </table>
            <hr />
            <button type="button">Button</button>
            "\u{00A0}"
            <button type="button" class="secondary">
                Secondary
            </button>
            "\u{00A0}"
            <button type="button" disabled>
                Disabled
            </button>
            <br />
            <br />
            <button type="button" class="ok">
                Affirmative
            </button>
            "\u{00A0}"
            <button type="button" class="danger">
                Negative
            </button>
            "\u{00A0}"
            <button class="caution">Cautionary</button>
            <br />
            <br />
            <input type="text" placeholder="Input" />
            "\u{00A0}"
            <input type="text" placeholder="Disabled" disabled />
            <br />
            <br />
            <textarea placeholder="Textarea"></textarea>
            <br />
            <br />
            <input type="checkbox" id="checkbox1" />
            <label for="checkbox1">Checkbox</label>
            "\u{00A0}"
            <input type="checkbox" id="checkbox2" disabled />
            <label for="checkbox2">Disabled checkbox</label>
            <br />
            <br />
            <input type="checkbox" class="toggle-switch" id="checkbox3" />
            <label for="checkbox3">Toggle switch</label>
            "\u{00A0}"
            <input type="checkbox" class="toggle-switch" id="checkbox4" disabled />
            <label for="checkbox4">Toggle switch</label>
            <br />
            <br />
            <input type="radio" name="radios" id="radio1" />
            <label for="radio1">Radio1</label>
            <br />
            <br />
            <input type="radio" name="radios" id="radio2" />
            <label for="radio2">Radio2</label>
            <br />
            <br />
            <input type="radio" name="radios" id="radio3" disabled />
            <label for="radio3">Disabled</label>
            <br />
            <br />
            <hr />
            <p>
                "Donec eleifend arcu nec lacus ultricies hendrerit. Aliquam fringilla odio risus, nec efficitur enim scelerisque sit amet. Integer tempor dui in nisi scelerisque, et consectetur velit eleifend. Integer porttitor tincidunt est id auctor. Maecenas sit amet hendrerit est. Phasellus in hendrerit enim. Fusce imperdiet viverra erat, et condimentum turpis ultricies non. Pellentesque blandit imperdiet mollis. Suspendisse metus dui, blandit ac purus eget, dignissim molestie dolor. Fusce dignissim ligula augue, et tristique nulla efficitur id."
            </p>
            <div class="card">
                <h1 class="caption">Card with Caption</h1>
                <p>
                    "I am not rightly able to apprehend the kind of confusion of ideas that
                    could have provoked such a question."
                </p>
                <div class="buttons-container">
                    <button class="secondary-button">Retry</button>
                    "\u{00A0}"
                    <button class="secondary-button">Cancel</button>
                </div>
            </div>
            <p>
                "Pellentesque pellentesque, dolor non ornare accumsan, magna sapien euismod orci, ac rutrum dui ex condimentum elit. Fusce egestas tincidunt libero eget suscipit. Phasellus in mauris sed sapien congue rutrum sed a ligula. Nulla faucibus ipsum nunc, eu dapibus orci euismod vitae. Aliquam laoreet dolor a semper suscipit. Cras at erat pellentesque, sollicitudin lorem at, mollis est. Curabitur sem diam, imperdiet sit amet gravida at, aliquet id neque. Duis aliquam magna eu tempor ultricies. Nulla mattis libero ac porttitor sodales. Morbi sit amet metus non erat maximus gravida. Mauris porta mi nec arcu facilisis, condimentum tristique erat efficitur. Aliquam erat volutpat. Curabitur vehicula felis ut arcu fermentum molestie. Pellentesque leo odio, luctus sed nunc vitae, viverra feugiat nisl. Phasellus sed est vel quam euismod commodo non eget urna."
            </p>
            <table class="banded-rows">
                <thead>
                    <tr>
                        <th>Column 1</th>
                        <th>Column 2</th>
                        <th>Color</th>
                        <th>Value</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>"Duis volutpat nulla"</td>
                        <td>"Suspendisse potenti. Sed a cursus erat."</td>
                        <td class="red">Red</td>
                        <td style="text-align: right">1.23456</td>
                    </tr>
                    <tr>
                        <td>"Praesent ac mauris"</td>
                        <td>"Sed maximus neque sed elit maximus ultrices ut sit amet velit."</td>
                        <td class="green">Green</td>
                        <td style="text-align: right">3.14159</td>
                    </tr>
                    <tr>
                        <td>"Vestibulum maximus mi non urna"</td>
                        <td>"Aenean a tristique arcu. Nullam volutpat sed nulla in porta."</td>
                        <td class="blue">Blue</td>
                        <td style="text-align: right">1.41421</td>
                    </tr>
                    <tr>
                        <td>"Duis volutpat nulla"</td>
                        <td>
                            "Maecenas posuere fermentum nibh et eleifend. Phasellus luctus fermentum enim et consectetur."
                        </td>
                        <td class="cyan">Cyan</td>
                        <td style="text-align: right">2.71828</td>
                    </tr>
                    <tr>
                        <td>Praesent ac mauris</td>
                        <td>Phasellus bibendum suscipit metus eu porttitor.</td>
                        <td class="purple">Purple</td>
                        <td style="text-align: right">42.00000</td>
                    </tr>
                    <tr>
                        <td>Vestibulum maximus mi non urna</td>
                        <td>Quisque ac dictum ipsum.</td>
                        <td class="yellow">Yellow</td>
                        <td style="text-align: right">0.70711</td>
                    </tr>
                </tbody>
            </table>
            <p>
                "Vivamus imperdiet diam elementum, scelerisque lacus non, feugiat odio. Vestibulum auctor ullamcorper lorem, eu ultrices urna bibendum sed. In hac habitasse platea dictumst. Nulla maximus sem a pulvinar auctor. In hac habitasse platea dictumst. Vivamus nisi augue, semper at ante at, convallis pretium magna. Integer sagittis mauris et eros fringilla, vitae sagittis nisl iaculis. Donec sed tincidunt nibh. Sed nibh lacus, suscipit et sodales in, tristique a turpis. Nulla pulvinar finibus mi, ac condimentum leo pellentesque sit amet. Donec condimentum eros et malesuada dapibus. Mauris laoreet eleifend tellus a pellentesque."
            </p>
            <div class="card caution">
                <h1 class="caption">Warning Card with Caption</h1>
                <p>
                    "Shields up. I recommend we transfer power to phasers and arm the photon torpedoes."
                </p>
                <div class="buttons-container">
                    <button class="caution">"Make it so"</button>
                    "\u{00A0}"
                </div>
            </div>
            <p>
                "Nam sed eros id est sodales tristique. Vestibulum id eros risus. Aliquam quis nulla elementum, bibendum ligula nec, tincidunt sem. Nulla viverra, ipsum sit amet sagittis pretium, sapien magna pellentesque orci, non elementum nunc enim quis odio. Nulla semper semper varius. Cras cursus nibh egestas convallis aliquam. Praesent efficitur sagittis enim vel sagittis."
            </p>
            <div class="card danger">
                <h1 class="caption">Error Card with Caption</h1>
                <p>"Divide by cucumber error, redo from start."</p>
                <div class="buttons-container">
                    <button class="danger">"Well poop"</button>
                    "\u{00A0}"
                </div>
            </div>
            <p>
                "Pellentesque pretium massa nisl, vel congue libero convallis non. Curabitur orci ipsum, maximus ac porta sodales, auctor non augue. Curabitur iaculis ligula orci, in malesuada sapien interdum id. Nam vel risus a mi imperdiet molestie. Nullam sed urna arcu. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Ut dictum eu nisi at rhoncus. Donec sed aliquam nunc. In euismod mauris eget gravida convallis. Praesent pharetra lorem ac nunc lobortis elementum. Sed dignissim condimentum nisi, non tristique arcu feugiat sit amet. Sed lobortis id nulla sed finibus. Quisque at augue a magna semper volutpat vitae ac nunc. Ut ut egestas sem."
            </p>
            <div class="card ok">
                <h1 class="caption">A pretty OK Card with Caption</h1>
                <p>"Everything is awesome! Everything is cool when you’re part of the team!"</p>
                <div class="buttons-container">
                    <button class="ok">Awesome!</button>
                    "\u{00A0}"
                </div>
            </div>
            <p>
                "Suspendisse mollis lectus sed rutrum cursus. Donec luctus est at rhoncus tempor. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras in convallis sapien. Morbi commodo ornare tellus, non laoreet felis ornare nec. Cras mi sapien, volutpat id nibh eget, tempus mattis lacus. Suspendisse sit amet sem at nibh blandit egestas sit amet et nisl. Vestibulum gravida et est eu tempor. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus."
            </p>
            <blockquote>
                <p>
                    "It is beneath the dignity of excellent men to waste their time in calculation when any peasant could do the work just as accurately with the aid of a machine."
                </p>
                <footer>"Gottfried Leibniz"</footer>
            </blockquote>
            <p>
                "Quisque dictum aliquet lorem. Morbi id odio est. Nulla non metus vitae neque placerat vestibulum ut quis lacus. Ut quis malesuada nibh, et ultrices lorem. Cras eget nisl rutrum, pretium sem ac, ornare magna. Suspendisse et vulputate dolor, nec tincidunt lacus. Nullam efficitur ex sed consectetur congue. Pellentesque semper magna quis urna malesuada, sed venenatis ex tempus. In mollis tempus fermentum. Proin tortor arcu, finibus id tellus at, tristique dictum elit. Fusce laoreet eu ipsum ac lacinia. Cras sit amet tellus lectus. Vivamus mattis ipsum ante, quis dapibus magna consequat elementum. Donec lacinia congue gravida."
            </p>
            <pre>"fn main() {\n" "  println!(\"Bonjour, le monde!\");\n" "  return 0;\n" "}\n"</pre>
            <p>
                "Mauris dolor quam, molestie sed lacus ut, laoreet interdum enim. Donec quis justo ut magna fringilla tristique sit amet non sapien. Duis sit amet eros posuere turpis tristique lacinia eget sit amet quam. Vivamus ullamcorper tincidunt dolor commodo faucibus. Suspendisse nec odio eleifend diam mattis mollis. Praesent pulvinar, urna eget ornare congue, ipsum leo finibus ligula, in egestas mi turpis semper lorem. Phasellus sodales lectus feugiat commodo sagittis. Quisque urna sem, pretium vitae magna ac, aliquet accumsan neque. Nulla lacinia lectus sed urna egestas congue. Suspendisse pharetra bibendum nibh a sagittis. Maecenas sed lacus in leo ullamcorper scelerisque. Morbi consequat pharetra metus et porta. Praesent metus ex, suscipit et consequat ac, accumsan nec tellus. Aenean dapibus faucibus nunc et aliquam."
            </p>
        </div>
    }
}
