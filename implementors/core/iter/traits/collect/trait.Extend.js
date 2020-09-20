(function() {var implementors = {};
implementors["bytes"] = [{"text":"impl Extend&lt;u8&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Extend&lt;&amp;'a u8&gt; for Bytes","synthetic":false,"types":[]},{"text":"impl Extend&lt;u8&gt; for BytesMut","synthetic":false,"types":[]},{"text":"impl&lt;'a&gt; Extend&lt;&amp;'a u8&gt; for BytesMut","synthetic":false,"types":[]}];
implementors["either"] = [{"text":"impl&lt;L, R, A&gt; Extend&lt;A&gt; for Either&lt;L, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: Extend&lt;A&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Extend&lt;A&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["evmap"] = [{"text":"impl&lt;K, V, M, S&gt; Extend&lt;(K, V)&gt; for WriteHandle&lt;K, V, M, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Eq + Hash + Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher + Clone,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Eq + ShallowCopy,<br>&nbsp;&nbsp;&nbsp;&nbsp;M: 'static + Clone,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["futures_util"] = [{"text":"impl&lt;Fut:&nbsp;Future&gt; Extend&lt;Fut&gt; for FuturesOrdered&lt;Fut&gt;","synthetic":false,"types":[]},{"text":"impl&lt;Fut&gt; Extend&lt;Fut&gt; for FuturesUnordered&lt;Fut&gt;","synthetic":false,"types":[]},{"text":"impl&lt;St:&nbsp;Stream + Unpin&gt; Extend&lt;St&gt; for SelectAll&lt;St&gt;","synthetic":false,"types":[]}];
implementors["hashbrown"] = [{"text":"impl&lt;K, V, S&gt; Extend&lt;(K, V)&gt; for HashMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V, S&gt; Extend&lt;(&amp;'a K, &amp;'a V)&gt; for HashMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Eq + Hash + Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; Extend&lt;T&gt; for HashSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, S&gt; Extend&lt;&amp;'a T&gt; for HashSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + Eq + Hash + Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["http"] = [{"text":"impl&lt;T&gt; Extend&lt;(Option&lt;HeaderName&gt;, T)&gt; for HeaderMap&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Extend&lt;(HeaderName, T)&gt; for HeaderMap&lt;T&gt;","synthetic":false,"types":[]}];
implementors["indexmap"] = [{"text":"impl&lt;K, V, S&gt; Extend&lt;(K, V)&gt; for IndexMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Hash + Eq,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, K, V, S&gt; Extend&lt;(&amp;'a K, &amp;'a V)&gt; for IndexMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Hash + Eq + Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: Copy,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; Extend&lt;T&gt; for IndexSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Hash + Eq,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;'a, T, S&gt; Extend&lt;&amp;'a T&gt; for IndexSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Hash + Eq + Copy + 'a,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl Extend&lt;TokenTree&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl Extend&lt;TokenStream&gt; for TokenStream","synthetic":false,"types":[]}];
implementors["serde_json"] = [{"text":"impl Extend&lt;(String, Value)&gt; for Map&lt;String, Value&gt;","synthetic":false,"types":[]}];
implementors["smallvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; Extend&lt;&lt;A as Array&gt;::Item&gt; for SmallVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl&lt;T, P&gt; Extend&lt;T&gt; for Punctuated&lt;T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Default,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, P&gt; Extend&lt;Pair&lt;T, P&gt;&gt; for Punctuated&lt;T, P&gt;","synthetic":false,"types":[]},{"text":"impl Extend&lt;Error&gt; for Error","synthetic":false,"types":[]}];
implementors["tinyvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; Extend&lt;&lt;A as Array&gt;::Item&gt; for ArrayVec&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array&gt; Extend&lt;&lt;A as Array&gt;::Item&gt; for TinyVec&lt;A&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()