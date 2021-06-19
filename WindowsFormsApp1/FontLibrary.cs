using System;
using System.Drawing;
using System.Drawing.Text;
using System.Runtime.InteropServices;
using HangulCraft.Properties;

namespace HangulCraft {
    public class FontLibrary {
        private static FontLibrary inst = new FontLibrary();
        private PrivateFontCollection privateFont = new PrivateFontCollection();
        public static FontFamily globalFont {
            get {
                return inst.privateFont.Families[0];
            }
        }

        public FontLibrary() {
            AddFontFromMemory();
        }

        [DllImport("gdi32.dll")]
        private static extern IntPtr AddFontMemResourceEx(IntPtr pbFont, uint cbFont, IntPtr pdv, [In] ref uint pcFonts);
        public void AddFontFromMemory() {
            byte[] fontData = Resources.GmarketSansTTFLight;
            IntPtr fontPtr = Marshal.AllocCoTaskMem(fontData.Length);
            Marshal.Copy(fontData, 0, fontPtr, fontData.Length);
            uint dummy = 0;
            privateFont.AddMemoryFont(fontPtr, Resources.GmarketSansTTFLight.Length);
            AddFontMemResourceEx(fontPtr, (uint) Resources.GmarketSansTTFLight.Length, IntPtr.Zero, ref dummy);
            Marshal.FreeCoTaskMem(fontPtr);
        }
    }
}