(function() {var implementors = {
"jf_plonk":[["impl&lt;'de, E: Pairing&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_plonk/proof_system/structs/struct.BatchProof.html\" title=\"struct jf_plonk::proof_system::structs::BatchProof\">BatchProof</a>&lt;E&gt;"],["impl&lt;'de, E: Pairing&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_plonk/proof_system/structs/struct.Proof.html\" title=\"struct jf_plonk::proof_system::structs::Proof\">Proof</a>&lt;E&gt;"]],
"jf_primitives":[["impl&lt;'de, P&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/signatures/schnorr/struct.KeyPair.html\" title=\"struct jf_primitives::signatures::schnorr::KeyPair\">KeyPair</a>&lt;P&gt;<span class=\"where fmt-newline\">where\n    P: Config,</span>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/signatures/bls_over_bn254/struct.VerKey.html\" title=\"struct jf_primitives::signatures::bls_over_bn254::VerKey\">VerKey</a>"],["impl&lt;'de, H&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/merkle_tree/hasher/struct.HasherNode.html\" title=\"struct jf_primitives::merkle_tree::hasher::HasherNode\">HasherNode</a>&lt;H&gt;<span class=\"where fmt-newline\">where\n    H: Digest,\n    Output&lt;H&gt;: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + for&lt;'a&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'a&gt;,</span>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/signatures/bls_over_bls12381/struct.BLSSignature.html\" title=\"struct jf_primitives::signatures::bls_over_bls12381::BLSSignature\">BLSSignature</a>"],["impl&lt;'de, E, H, I, Arity, T&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/merkle_tree/universal_merkle_tree/struct.UniversalMerkleTree.html\" title=\"struct jf_primitives::merkle_tree::universal_merkle_tree::UniversalMerkleTree\">UniversalMerkleTree</a>&lt;E, H, I, Arity, T&gt;<span class=\"where fmt-newline\">where\n    E: <a class=\"trait\" href=\"jf_primitives/merkle_tree/trait.Element.html\" title=\"trait jf_primitives::merkle_tree::Element\">Element</a> + CanonicalSerialize + CanonicalDeserialize,\n    H: <a class=\"trait\" href=\"jf_primitives/merkle_tree/trait.DigestAlgorithm.html\" title=\"trait jf_primitives::merkle_tree::DigestAlgorithm\">DigestAlgorithm</a>&lt;E, I, T&gt;,\n    I: <a class=\"trait\" href=\"jf_primitives/merkle_tree/trait.Index.html\" title=\"trait jf_primitives::merkle_tree::Index\">Index</a> + CanonicalSerialize + CanonicalDeserialize,\n    Arity: <a class=\"trait\" href=\"https://docs.rs/typenum/1.16.0/typenum/marker_traits/trait.Unsigned.html\" title=\"trait typenum::marker_traits::Unsigned\">Unsigned</a>,\n    T: <a class=\"trait\" href=\"jf_primitives/merkle_tree/trait.NodeValue.html\" title=\"trait jf_primitives::merkle_tree::NodeValue\">NodeValue</a>,</span>"],["impl&lt;'de, P&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/signatures/schnorr/struct.VerKey.html\" title=\"struct jf_primitives::signatures::schnorr::VerKey\">VerKey</a>&lt;P&gt;<span class=\"where fmt-newline\">where\n    P: Config,</span>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/signatures/bls_over_bn254/struct.Signature.html\" title=\"struct jf_primitives::signatures::bls_over_bn254::Signature\">Signature</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/signatures/bls_over_bn254/struct.BLSOverBN254CurveSignatureScheme.html\" title=\"struct jf_primitives::signatures::bls_over_bn254::BLSOverBN254CurveSignatureScheme\">BLSOverBN254CurveSignatureScheme</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/aead/struct.EncKey.html\" title=\"struct jf_primitives::aead::EncKey\">EncKey</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/signatures/bls_over_bn254/struct.SignKey.html\" title=\"struct jf_primitives::signatures::bls_over_bn254::SignKey\">SignKey</a>"],["impl&lt;'de, E, H, I, Arity, T&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/merkle_tree/light_weight/struct.LightWeightMerkleTree.html\" title=\"struct jf_primitives::merkle_tree::light_weight::LightWeightMerkleTree\">LightWeightMerkleTree</a>&lt;E, H, I, Arity, T&gt;<span class=\"where fmt-newline\">where\n    E: <a class=\"trait\" href=\"jf_primitives/merkle_tree/trait.Element.html\" title=\"trait jf_primitives::merkle_tree::Element\">Element</a> + CanonicalSerialize + CanonicalDeserialize,\n    H: <a class=\"trait\" href=\"jf_primitives/merkle_tree/trait.DigestAlgorithm.html\" title=\"trait jf_primitives::merkle_tree::DigestAlgorithm\">DigestAlgorithm</a>&lt;E, I, T&gt;,\n    I: <a class=\"trait\" href=\"jf_primitives/merkle_tree/trait.Index.html\" title=\"trait jf_primitives::merkle_tree::Index\">Index</a> + CanonicalSerialize + CanonicalDeserialize,\n    Arity: <a class=\"trait\" href=\"https://docs.rs/typenum/1.16.0/typenum/marker_traits/trait.Unsigned.html\" title=\"trait typenum::marker_traits::Unsigned\">Unsigned</a>,\n    T: <a class=\"trait\" href=\"jf_primitives/merkle_tree/trait.NodeValue.html\" title=\"trait jf_primitives::merkle_tree::NodeValue\">NodeValue</a>,</span>"],["impl&lt;'de, P&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/signatures/schnorr/struct.Signature.html\" title=\"struct jf_primitives::signatures::schnorr::Signature\">Signature</a>&lt;P&gt;<span class=\"where fmt-newline\">where\n    P: Config,</span>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/aead/struct.Ciphertext.html\" title=\"struct jf_primitives::aead::Ciphertext\">Ciphertext</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/aead/struct.KeyPair.html\" title=\"struct jf_primitives::aead::KeyPair\">KeyPair</a>"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/signatures/bls_over_bls12381/struct.BLSSignKey.html\" title=\"struct jf_primitives::signatures::bls_over_bls12381::BLSSignKey\">BLSSignKey</a>"],["impl&lt;'de, F: PrimeField&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/signatures/schnorr/struct.SignKey.html\" title=\"struct jf_primitives::signatures::schnorr::SignKey\">SignKey</a>&lt;F&gt;"],["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/signatures/bls_over_bls12381/struct.BLSVerKey.html\" title=\"struct jf_primitives::signatures::bls_over_bls12381::BLSVerKey\">BLSVerKey</a>"],["impl&lt;'de, E, H, I, Arity, T&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_primitives/merkle_tree/append_only/struct.MerkleTree.html\" title=\"struct jf_primitives::merkle_tree::append_only::MerkleTree\">MerkleTree</a>&lt;E, H, I, Arity, T&gt;<span class=\"where fmt-newline\">where\n    E: <a class=\"trait\" href=\"jf_primitives/merkle_tree/trait.Element.html\" title=\"trait jf_primitives::merkle_tree::Element\">Element</a> + CanonicalSerialize + CanonicalDeserialize,\n    H: <a class=\"trait\" href=\"jf_primitives/merkle_tree/trait.DigestAlgorithm.html\" title=\"trait jf_primitives::merkle_tree::DigestAlgorithm\">DigestAlgorithm</a>&lt;E, I, T&gt;,\n    I: <a class=\"trait\" href=\"jf_primitives/merkle_tree/trait.Index.html\" title=\"trait jf_primitives::merkle_tree::Index\">Index</a> + CanonicalSerialize + CanonicalDeserialize,\n    Arity: <a class=\"trait\" href=\"https://docs.rs/typenum/1.16.0/typenum/marker_traits/trait.Unsigned.html\" title=\"trait typenum::marker_traits::Unsigned\">Unsigned</a>,\n    T: <a class=\"trait\" href=\"jf_primitives/merkle_tree/trait.NodeValue.html\" title=\"trait jf_primitives::merkle_tree::NodeValue\">NodeValue</a>,</span>"]],
"jf_utils":[["impl&lt;'de&gt; <a class=\"trait\" href=\"https://docs.rs/serde/1.0.163/serde/de/trait.Deserialize.html\" title=\"trait serde::de::Deserialize\">Deserialize</a>&lt;'de&gt; for <a class=\"struct\" href=\"jf_utils/struct.CanonicalBytes.html\" title=\"struct jf_utils::CanonicalBytes\">CanonicalBytes</a>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()