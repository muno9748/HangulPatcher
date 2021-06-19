using System;
using System.Collections.Generic;
using System.Drawing;
using System.Drawing.Text;
using System.Runtime.InteropServices;

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
            int fontLength = Properties.Resources.GmarketSansTTFLight.Length;
            byte[] fontdata = Properties.Resources.GmarketSansTTFLight;

            IntPtr data = Marshal.AllocCoTaskMem(fontLength);
            Marshal.Copy(fontdata, 0, data, fontLength);

            privateFont.AddMemoryFont(data, fontLength);
        }

    }
}