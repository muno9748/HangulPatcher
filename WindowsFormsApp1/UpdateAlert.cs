using System;
using System.Runtime.InteropServices;
using System.Windows.Forms;
using System.Diagnostics;

namespace HangulCraft {
    public partial class UpdateAlert: Form {
        public const int WM_NCLBUTTONDOWN = 0xA1;
        public const int HT_CAPTION = 0x2;

        public string link = "";

        [DllImport("user32.dll")]
        public static extern bool ReleaseCapture();
        [DllImport("user32.dll")]
        public static extern int SendMessage(IntPtr hWnd, int Msg, int wParam, int lParam);

        public UpdateAlert() {
            InitializeComponent();
        }

        private void Exit_Click(object sender, EventArgs e) {
            Form1.Terminate();
            Dispose();
        }

        private void MoveWindow(object sender, MouseEventArgs e) {
            if (e.Button == MouseButtons.Left) {
                ReleaseCapture();
                SendMessage(Handle, WM_NCLBUTTONDOWN, HT_CAPTION, 0);
            }
        }

        private void downloadLink_LinkClicked(object sender, LinkLabelLinkClickedEventArgs e) {
            Process.Start(link);
        }
    }
}
