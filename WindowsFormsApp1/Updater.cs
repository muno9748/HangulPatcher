using System;
using System.IO;
using System.Net;
using System.Windows.Forms;

namespace HangulCraft {
    public class Updater {
        public static readonly string EndPoint = "https://raw.githubusercontent.com/muno9748/HangulPatcher/master/LatestVersion";
        public static readonly string currentVersion = "1.2.0";
        public static string latestVersion;
        public static void CheckUpdate() {
            string responseText = String.Empty;

            HttpWebRequest request = (HttpWebRequest) WebRequest.Create(EndPoint);
            request.Method = "GET";
            request.Timeout = 30 * 1000;

            using (HttpWebResponse resp = (HttpWebResponse) request.GetResponse()) {
                HttpStatusCode status = resp.StatusCode;
                Console.WriteLine(status);

                Stream respStream = resp.GetResponseStream();
                using (StreamReader sr = new StreamReader(respStream)) {
                    responseText = sr.ReadToEnd();
                }
            }

            int version;
            int current;
            Int32.TryParse(responseText.Trim().Replace(".", ""), out version);
            Int32.TryParse(currentVersion.Replace(".", ""), out current);

            latestVersion = responseText.Trim();

            Form1.AddLog($"Current Version: {currentVersion}");
            Form1.AddLog($"Latest Version: {responseText.Trim()}");

            if (current < version) {
                Form1.paused = true;
                Form1.pause.Enabled = false;
                Form1.pause.Text = "재시작";
                Form1.AddLog("Hook Process terminated.");
                Form1.AddLog("You are using outdated version!");

                UpdateAlert form = new UpdateAlert();

                form.link = $"https://github.com/muno9748/HangulPatcher/releases/tag/{latestVersion}";

                form.BringToFront();
                form.WindowState = FormWindowState.Normal;
                form.ShowDialog(Form1.instance);
            }
        }
    }
}
