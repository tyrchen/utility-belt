(function() {var implementors = {};
implementors["fake_socket"] = [{"text":"impl&lt;T, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"fake_socket/struct.ReceiverStream.html\" title=\"struct fake_socket::ReceiverStream\">ReceiverStream</a>&lt;T, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["fake_socket::ReceiverStream"]},{"text":"impl&lt;T, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"fake_socket/struct.SenderSink.html\" title=\"struct fake_socket::SenderSink\">SenderSink</a>&lt;T, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["fake_socket::SenderSink"]},{"text":"impl&lt;T, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"fake_socket/struct.FakeSocket.html\" title=\"struct fake_socket::FakeSocket\">FakeSocket</a>&lt;T, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["fake_socket::FakeSocket"]}];
implementors["simple_pipeline"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"simple_pipeline/enum.PipelineError.html\" title=\"enum simple_pipeline::PipelineError\">PipelineError</a>","synthetic":true,"types":["simple_pipeline::PipelineError"]},{"text":"impl&lt;Ctx&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"simple_pipeline/enum.PlugResult.html\" title=\"enum simple_pipeline::PlugResult\">PlugResult</a>&lt;Ctx&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Ctx: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["simple_pipeline::PlugResult"]},{"text":"impl&lt;Ctx&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"simple_pipeline/struct.Pipeline.html\" title=\"struct simple_pipeline::Pipeline\">Pipeline</a>&lt;Ctx&gt;","synthetic":true,"types":["simple_pipeline::Pipeline"]},{"text":"impl&lt;Ctx&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"simple_pipeline/struct.PipelineResponse.html\" title=\"struct simple_pipeline::PipelineResponse\">PipelineResponse</a>&lt;Ctx&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Ctx: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["simple_pipeline::PipelineResponse"]}];
implementors["simple_pubsub"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"simple_pubsub/struct.Broadcaster.html\" title=\"struct simple_pubsub::Broadcaster\">Broadcaster</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.58.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["simple_pubsub::broadcaster::Broadcaster"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()