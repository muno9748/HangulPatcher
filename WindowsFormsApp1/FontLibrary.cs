using System;
using System.Drawing;
using System.Drawing.Text;
using System.Runtime.InteropServices;
using HangulCraft.Properties;

namespace HangulCraft {
    public class FontLibrary {
        private static FontLibrary inst = new FontLibrary();
        public PrivateFontCollection privateFont = new PrivateFontCollection();
        public static FontFamily[] Families {
            get {
                return inst.privateFont.Families;
            }
        }

        public FontLibrary() {
            AddFontFromMemory();
        }

        private void AddFontFromMemory() {
            int fontLength = Resources.GmarketSansTTFLight.Length;
            byte[] fontdata = Resources.GmarketSansTTFLight;

            IntPtr data = Marshal.AllocCoTaskMem(fontLength);
            Marshal.Copy(fontdata, 0, data, fontLength);

            privateFont.AddMemoryFont(data, fontLength);
        }

    }
}