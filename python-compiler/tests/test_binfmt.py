import hashlib, os, sys, tempfile, unittest
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))
from aegis_policy import build_dummy, verify_bin


class T(unittest.TestCase):
    def test_valid_aligned_deterministic(self):
        with tempfile.TemporaryDirectory() as t:
            a = build_dummy(os.path.join(t, "a.bin"))
            b = build_dummy(os.path.join(t, "b.bin"))
            self.assertEqual(hashlib.sha256(a).digest(), hashlib.sha256(b).digest())
            info = verify_bin(a)
            self.assertEqual(info["proj_bytes"] % 64, 0)
            self.assertEqual((64 + info["proj_bytes"]) % 64, 0)  # centroids start 64-aligned
            self.assertEqual(a[:4], b"SGEA")  # 0x41454753 little-endian

    def test_corruption_rejected(self):
        with tempfile.TemporaryDirectory() as t:
            a = build_dummy(os.path.join(t, "a.bin"))
        for bad in (a[:-1], b"\x00" + a[1:], a[:10]):
            with self.assertRaises(ValueError):
                verify_bin(bad)


if __name__ == "__main__":
    unittest.main()
