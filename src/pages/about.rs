use leptos::prelude::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="space-y-5">
            <h1 class="text-3xl">"About me"</h1>
            <p>
                "I operate a knitting machine in a factory in England. Every morning I walk through a door that has nothing to do with who I am."
            </p>
            <p>
                " The work is repetitive. The people are difficult. Every shift I spend there is a shift spent in someone else's story. This is not my story. But it's where I am right now, and I'm not going to pretend otherwise. "
            </p>
            <p>
                " My name is Patryk. I was born in Poland. I grew up in a place where your options were visible from your front door — and they weren't many. School didn't go well for me. Not because I wasn't intelligent. I know that now. But I didn't know it then. All I knew was that exams were hard, reading was slow, and the system had a very specific idea of what smart looked like. I didn't look like it. "
            </p>
            <p>
                " I've been on my own since I was seventeen. While other people my age were figuring out who they were in the safety of a family home, I was figuring out how to pay rent. How to feed myself. How to make decisions that most people don't face until their mid-twenties. I didn't have the luxury of a slow transition into adulthood. I had bills, and I had myself, and that was it. It forced something in me that has never gone away — a self-reliance that runs so deep I sometimes forget it's unusual. I don't wait for someone to solve my problems. I don't expect a safety net. I build things because nobody is coming to build them for me. "
            </p>
            <p>
                " So I joined the Polish Armed Forces. I drove tanks. I maintained them. I learned something in the military that I've carried with me ever since — that a machine is not a single thing. It's a system. Hydraulics feeding into electronics feeding into mechanics feeding into communications. If you don't see the connections between the parts, you don't understand the machine. You just operate it. I didn't want to just operate things. I wanted to understand them. "
            </p>
            <p>
                " In 2011, I left Poland. I was young. I barely spoke English. I didn't have a plan. I had a feeling — that what was behind me wasn't enough, and that whatever was ahead of me, even if I couldn't name it, was worth walking toward. "
            </p>
            <p>
                " I landed in the UK and I started from nothing. Not \"startingover \" the way people say it casually. Actually nothing. No career. No language fluency. No network. No qualifications that meant anything here. Just time and the willingness to use it. But that wasn't new. I'd been starting from nothing since I was seventeen. "
            </p>
            <p>
                " I met Luiza. And she changed everything. Not in the dramatic, cinematic way people talk about love changing things. In the real way. She believed in things. She believed in building things. Together we opened a plant-based café in Leicester called Healthy Louisa. "
            </p>
            <p>
                " The first year almost killed us. Not metaphorically. Financially, physically, emotionally — we were on the edge. The customers weren't coming. The money was running out. The sixteen-hour days were piling up with nothing to show for them. We'd open the doors in the morning and just wait. Watching the street. Counting the minutes between customers. Wondering if we'd made the worst decision of our lives. "
            </p>
            <p>
                " We could have closed then. Most people would have. Instead, we pivoted. We looked at what wasn't working, we stripped it back, we rebuilt the approach. And it turned. Slowly at first, then properly. The five-star reviews started coming. People came back. They brought other people. What was nearly a grave became something we were proud of. "
            </p>
            <p>
                " That year taught us the most important lesson I've ever learned: you don't wait for results. You build them. Nobody is coming to rescue your business. Nobody is coming to rescue your career. Nobody is coming to hand you the life you want. You look at what's broken, you fix it, you keep going. That lesson lives in everything I do now — in how I approach code, in how I approach the job search, in how I approach this site. "
            </p>
            <p>
                " We closed the café voluntarily. People don't understand that. They hear \"closed \" and think \"failed. \" We didn't fail. We completed it. It taught us what it had to teach and we moved on. Closing something you built with your hands and your name is one of the hardest things a person can do. It's harder than building it, because building feels like progress and closing feels like loss, even when it isn't. "
            </p>
            <p>
                " After that, I found programming. And I need to be precise about what happened, because it wasn't just \"oh , I liked it.\" Something fundamental shifted in my brain. "
            </p>
            <p>
                " For my entire life, the way I think — in systems, in connections, in patterns that link things other people see as separate — had no outlet. I could feel it. I could see how things connected. But I couldn't express it. Not in words fast enough, not in writing easily enough, not in any medium that the world gave me access to. My brain was a V8 engine connected to bicycle wheels. "
            </p>
            <p>" Code was the first set of wheels that matched the engine. "</p>
            <p>
                " But not all code. I started with Ruby on Rails. It was elegant. The conventions were clean. The idea of \"convention over configuration \" was appealing — here's how things are done, just follow the pattern. I could build things quickly. But something didn't feel right. There was a looseness underneath the elegance. The magic was real magic — things happened that I couldn't trace. Methods appeared from nowhere. The framework made decisions behind my back. When things worked, it felt effortless. When things broke, I couldn't always find where or why because the system was doing things I hadn't asked it to do. I was productive but I wasn't confident. There's a difference. "
            </p>
            <p>
                " I moved to JavaScript. I worked with Express — building backends, handling routes, wiring up middleware. It was more explicit than Rails. I could see the request come in and the response go out. But JavaScript itself was the problem. Loose types. Silent coercions. The kind of language that says \"sure , that's fine \" when it shouldn't be, and then breaks at runtime in ways you never predicted. I was writing code that ran but I didn't trust it. My working memory couldn't hold all the invisible rules that JavaScript expected me to track. "
            </p>
            <p>
                " I tried Vue.js and SvelteKit. Both were steps in the right direction. SvelteKit especially — the compiler-first approach resonated with me. The syntax was cleaner. The mental model was simpler. I genuinely liked building with it. But underneath both of them was still JavaScript. Still dynamic. Still capable of surprising me. Still asking my brain to hold things that my brain couldn't hold. Then TypeScript gave me types, guardrails, structure. But the type system was optional in practice. You could escape it with \"any . \" It was a seatbelt in a car with no brakes — better than nothing, but not enough to make me feel safe. "
            </p>
            <p>
                " Through all of this — Rails, Express, Vue, SvelteKit, TypeScript — I could build things. I shipped projects. I solved problems. But there was always a voice in the back of my mind saying \"this will break and you won't know where. \" That voice wasn't paranoia. It was my brain telling me that these languages required a kind of cognitive bookkeeping that my working memory couldn't sustain. I didn't have the vocabulary for it then. I just felt unsure. Fragile. Like I was a fraud who hadn't been caught yet. "
            </p>
            <p>" Then I found Rust. "</p>
            <p>" And for the first time in my life, I felt solid. "</p>
            <p>
                " Rust doesn't trust you. Rust verifies. Every type is explicit. Every error must be handled. Every ownership transfer is visible in the code. The compiler doesn't let you write fragile software. It fights you. It argues with you. It says \"no \" a hundred times before it says \"okay, I believe you. \" And when it says okay, you know — actually know, not hope — that the code works. "
            </p>
            <p>" For a brain like mine, that isn't a burden. It's liberation. "</p>
            <p>
                " My working memory is 77. Below average. I cannot hold five invisible rules in my head while writing code. Ruby and JavaScript and Vue and Svelte all asked me to do exactly that. Rust doesn't ask. Rust puts the rules in the code where I can see them. The compiler remembers what my brain can't. The language compensates for exactly the thing I struggle with, and it rewards exactly the thing I'm best at — deep understanding of how systems connect. "
            </p>
            <p>
                " I didn't know any of this when I chose Rust. I chose it on instinct. My brain recognised something in it before I could articulate what. It felt like the language was built for the way I think. I now know why. "
            </p>
            <p>
                " A few months ago, I had a cognitive assessment. A formal one. The kind where someone sits you down for hours and measures how your brain actually works. Not what you know — how you process. "
            </p>
            <p>" The results explained my entire life. "</p>
            <img class="h-20" src="" alt="" />
            <p>" Silent reading comprehension: 129. 97th percentile. In my second language. "</p>
            <p>" Vocabulary: 111. Strong. Precise. "</p>
            <p>" Working memory: 77. Below average. "</p>
            <p>" Decoding speed: 56. Bottom percentile. "</p>
            <p>" Sharp peaks. Deep valleys. Same brain. "</p>
            <p>
                " But here's the thing people don't understand about that comprehension score. I don't read the way most people read. Most people decode — they see letters, assemble them into sounds, turn sounds into words, and extract meaning. It's fast. It's automatic. They don't even think about it. "
            </p>
            <p>
                " I can't do that. My decoding score is 56. The mechanical pipeline that most people take for granted barely works for me. So my brain built a different system. "
            </p>
            <p>
                " I memorise words whole. Not letter by letter, not sound by sound — as complete visual shapes. When I see a word I've seen before, I don't decode it. I recognise it. The way you recognise a face. Instantly, as a whole, without analysing the individual features. And each word I recognise gets placed into the context of what I've already read. Meaning builds not from the bottom up — letters to sounds to words to sentences — but from the top down. Context first. Pattern first. The meaning of the sentence helps me recognise the next word, which feeds back into the meaning, which helps me recognise the next one. "
            </p>
            <p>
                " It's slow. It's expensive. Every new word is a problem to be solved because I can't just sound it out. I have to encounter it enough times in enough contexts to absorb it as a shape with meaning attached. But once a word is in my system, it's in deep. It's not just a definition — it's connected to every context I've ever seen it in. That's why my vocabulary score is 111 and my comprehension is 129 despite a decoding score that should make reading nearly impossible. My brain didn't accept the bottleneck. It engineered around it. Without anyone teaching it how. Without me even knowing it was doing it. "
            </p>
            <p>
                " This is how I read code too. I don't parse Rust syntax character by character. I recognise patterns. A match block is a visual shape. A struct definition is a shape. An impl block is a shape. I see the structure before I see the details, and the structure tells me what the details mean before I've read them. It's the same compensatory system, just applied to a different language. "
            </p>
            <p>
                " Rust rewards this. The syntax is consistent. The patterns are predictable. The shapes repeat. Once you've memorised the visual grammar of Rust, you can read it the way I read everything — as recognised shapes placed into context, meaning flowing top-down from structure to detail. Dynamic languages don't have this consistency. The shapes shift. The patterns break. My system can't lock on. "
            </p>
            <p>
                " I have been running on hard mode my entire life without knowing the difficulty was turned up. "
            </p>
            <p>
                " The assessor called it a \"spikyprofile.\" Sharp peaks and deep valleys on the same graph. Not a flat line of average. Not a smooth curve of general ability. Spikes. Jagged. Uneven. Extreme in both directions. "
            </p>
            <p>
                " That's why this site is called spikyprofile.dev. It's not a brand I invented. It's a clinical description that I reclaimed. "
            </p>
            <p>
                " Right now, I'm studying BSc Computer Science at the University of London part-time. It's a six-year commitment while working full-time. I study at night. I build at night. I've designed a custom split ergonomic keyboard with 80 hall effect sensors, a STM32 microcontroller, haptic feedback, and a four-layer PCB. I plan to write the firmware in Rust. "
            </p>
            <p>
                " I'm building a full-stack coaching platform for my fiancée's business — Wildly Magnetic. It's a real application for a real user. Leptos frontend, Axum backend, authentication, payments, the full stack. Someone trusted me to build something for their livelihood. That's not a tutorial project. "
            </p>
            <p>
                " I run my own infrastructure. Not because I have to. Because I need to understand the whole system. Proxmox virtualisation on bare metal hardware. Nginx reverse proxy. GitHub Actions for continuous deployment. Stalwart mail server. SOGo webmail. Plausible analytics. No managed services. No cloud dashboards. Everything built from the ground up by one person, at night, after work, because that's how my brain learns — by owning every layer. "
            </p>
            <p>
                " I've applied to over 40 junior developer positions. I've been rejected by all of them. IBM. Counter Terrorism Policing. 1Password. Dozens of others. My CV says \"Knitting Machine Operator. \" The ATS reads that and moves on. It doesn't know about the tanks. It doesn't know about the café. It doesn't know about the infrastructure. It doesn't know about the brain that understands systems at the 97th percentile but decodes text at the 4th. The filter is too stupid to see what I am. But the filter is all that stands between me and the career I'm building toward. "
            </p>
            <p>
                " I don't stop. I have never stopped. Not when I was seventeen and alone. Not when I left Poland with nothing. Not when I learned English with a brain that makes reading harder than it should be. Not when the first year of the café almost broke us. Not when the fortieth rejection came in. Not when the forty-first did. "
            </p>
            <p>" Giving up is not in my DNA. It never has been. "</p>
            <p>
                " This site is my untimed exam. No clock. No word limit. No algorithm deciding whether I'm worth a look. Just my work. My thinking. My way of understanding Rust and systems and code and the world. In the open. For anyone who wants to see it. "
            </p>
            <p>" If you learn the way I learn, welcome. "</p>
            <p>"You found the right place."</p>
        </div>
    }
}
