<article class="ecs">
<h3>What is ECS</h3>
<p>
The term ECS is a shorthand for Entity-component system.</p>
<p>
These are the three core concepts:</p>
<ul>
<li>Each <b>entity</b> is associated with some <b>components</b>.</li>
<li>Those entities and components are processed by systems. This way, you have your data (components) completely separated from the behaviour (systems).</li>
<li>An entity just logically groups components; so a Velocity component can be applied to the Position component of the same entity.</li>
</ul>

<hr>
<p>
 <b>An ECS has the following characteristics:</b>
</p>
<ul>
<li>It has entities, which are unique identifiers </li>
<li>It has components, which are plain data types without behavior</li>
<li>Benefits Entities can contain zero or more components</li>
<li>Entities can change components dynamically</li>
<li>It has systems, which are functions matched with entities that have a certain set of components.</li>
</ul>
</article>
<hr>
<aside>
<h5>Additional Resources</h5>
<dl>
<ol>
<dt><a href="https://crates.io/crates/specs">Specs - ECS written in Rust (crate)</a></dt>
<dd>Specs is an Entity-Component System written in Rust.</dd>
<dt><a href="https://adventures.michaelfbryan.com/posts/ecs-outside-of-games/#inheritance-isnt-always-the-best-tool-for-the-job">Using the ECS Pattern Outside of Game Engines</a></dt>
<dd>Tutorial for creating an ECS-based CAD Library</dd>
<dt><a href="https://specs.amethyst.rs/docs/tutorials/">Specs Book</a></dt>
<dd>Specs is an ECS library that allows parallel system execution, with both low overhead and high flexibility, different storage types and a type-level system data model.</dd>
<dd>It is mainly used for games and simulations, where it allows to structure code using composition over inheritance.</dd>
</ol>
</dl>
</aside>

