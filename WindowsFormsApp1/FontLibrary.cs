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
            List<byte[]> fonts = new List<byte[]>();
            fonts.Add(Properties.Resources.GmarketSansTTFLight);

            foreach (byte[] font in fonts) {
                IntPtr fontBuffer = Marshal.AllocCoTaskMem(font.Length);
                Marshal.Copy(font, 0, fontBuffer, font.Length);
                privateFont.AddMemoryFont(fontBuffer, font.Length);
            }
        }

    }
}