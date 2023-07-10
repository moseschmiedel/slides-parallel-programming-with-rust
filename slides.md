---
theme: academic
coverAuthor: Mose Schmiedel
coverDate: 10/7/2023
coverBackgroundUrl: /SunsetTracks.jpg
coverBackgroundSource: >-
  Arne Hückelheim, CC BY-SA 3.0
  <https://creativecommons.org/licenses/by-sa/3.0>, via Wikimedia Commons
coverBackgroundSourceUrl: https://upload.wikimedia.org/wikipedia/commons/a/a9/SunsetTracks.JPG
fonts:
  sans: Source Sans Pro
  serif: Source Serif Pro
  mono: JuliaMono
class: text-center text-white
highlighter: shiki
lineNumbers: true
info: |
  ## Slides for talk about 'parallel programming in Rust'
drawings:
  persist: false
title: parallel programming in Rust
layout: cover
hideInToc: true
---

<div class="text-6xl">

parallel programming in Rust
</div>


---
hideInToc: true
---

# outline

<Toc />


---
src: ./pages/importance-of-parallel-programming.md
---

---
src: ./pages/difficulties-of-parallel-execution.md
---

---
src: ./pages/parallelism-in-rust.md
---

---
hideInToc: true
---

<div class="flex flex-column w-full h-full items-center justify-around">
<div class="text-3xl text-center font-bold">
    <div v-click class="mt-8">
    Parallel programming is essential for scaling up applications.
    </div>
    <div v-click class="mt-16">
    Rust enables us to write parallel applications without fear.
    </div>
</div>
</div>

<!--
- physical limitations enforce usage of parallel programming to increase performance of computer systems
- the compiler of Rust is a powerful code analysis tool, that helps to write correct parallel applications
-->

---
layout: index
indexEntries:
    -
        title: Integrated Circuit - Cole L, CC BY-SA 2.0 <https://creativecommons.org/licenses/by-sa/2.0>, via Wikimedia Commons
        uri: https://upload.wikimedia.org/wikipedia/commons/0/07/AME_AS3336G_Precision_Multiple_Analog_Switch_%2849838375532%29.png
indexRedirectType: external
hideInToc: true
---
# index

---
layout: end
---
# Q&A

<div class="mt-8">

### further resources:
- [parallel programming] [Seven Concurrency Models in Seven Weeks](https://pragprog.com/titles/pb7con/seven-concurrency-models-in-seven-weeks/)
- [Rust] https://doc.rust-lang.org/book/
</div>

<style>
li {
    list-style-type: none;
}
</style>
