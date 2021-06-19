using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Windows.Forms;
using HangulCraft.Properties;

namespace HangulCraft {
    public partial class Form1: Form {
        public Form1() {
            InitializeComponent();
        }

        #region #DllImport

        [DllImport("user32.dll")]
        static extern IntPtr SetWindowsHookEx(int idHook, LowLevelKeyboardProc callback, IntPtr hInstance, uint threadId);

        [DllImport("user32.dll")]
        static extern bool UnhookWindowsHookEx(IntPtr hInstance);

        [DllImport("user32.dll")]
        static extern IntPtr CallNextHookEx(IntPtr idHook, int nCode, int wParam, IntPtr lParam);

        [DllImport("kernel32.dll")]
        static extern IntPtr LoadLibrary(string lpFileName);

        private delegate IntPtr LowLevelKeyboardProc(int nCode, IntPtr wParam, IntPtr lParam);

        const int WH_KEYBOARD_LL = 13;
        const int WM_KEYDOWN = 0x100;

        private readonly LowLevelKeyboardProc _proc = hookProc;

        private static IntPtr hhook = IntPtr.Zero;

        public void SetHook() {
            IntPtr hInstance = LoadLibrary("User32");
            hhook = SetWindowsHookEx(WH_KEYBOARD_LL, _proc, hInstance, 0);
        }

        public static void UnHook() {
            UnhookWindowsHookEx(hhook);
        }

        public const int WM_NCLBUTTONDOWN = 0xA1;
        public const int HT_CAPTION = 0x2;

        [DllImport("user32.dll")]
        public static extern int SendMessage(IntPtr hWnd, int Msg, int wParam, int lParam);
        [DllImport("user32.dll")]
        public static extern bool ReleaseCapture();

        #endregion
        #region #HangulParser
        private static readonly string m초성Tbl = "ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ";
        private static readonly string m중성Tbl = "ㅏㅐㅑㅒㅓㅔㅕㅖㅗㅘㅙㅚㅛㅜㅝㅞㅟㅠㅡㅢㅣ";
        private static readonly string m종성Tbl = " ㄱㄲㄳㄴㄵㄶㄷㄹㄺㄻㄼㄽㄾㄿㅀㅁㅂㅄㅅㅆㅇㅈㅊㅋㅌㅍㅎ";
        private static readonly string mDbl종성Tbl = "ㄲㄳㄵㄶㄺㄻㄼㄽㄾㄿㅀㅄ";
        private static readonly ushort mUniCode한글Base = 0xAC00;
        private static readonly Dictionary<Keys, char> hangulMap = new Dictionary<Keys, char>() {
            { Keys.Q, 'ㅂ' },
            { Keys.W, 'ㅈ' },
            { Keys.E, 'ㄷ' },
            { Keys.R, 'ㄱ' },
            { Keys.T, 'ㅅ' },
            { Keys.Y, 'ㅛ' },
            { Keys.U, 'ㅕ' },
            { Keys.I, 'ㅑ' },
            { Keys.O, 'ㅐ' },
            { Keys.P, 'ㅔ' },
            { Keys.A, 'ㅁ' },
            { Keys.S, 'ㄴ' },
            { Keys.D, 'ㅇ' },
            { Keys.F, 'ㄹ' },
            { Keys.G, 'ㅎ' },
            { Keys.H, 'ㅗ' },
            { Keys.J, 'ㅓ' },
            { Keys.K, 'ㅏ' },
            { Keys.L, 'ㅣ' },
            { Keys.Z, 'ㅋ' },
            { Keys.X, 'ㅌ' },
            { Keys.C, 'ㅊ' },
            { Keys.V, 'ㅍ' },
            { Keys.B, 'ㅠ' },
            { Keys.N, 'ㅜ' },
            { Keys.M, 'ㅡ' },
        };
        private static readonly Dictionary<Keys, char> shiftHangulMap = new Dictionary<Keys, char>() {
            { Keys.Q, 'ㅃ' },
            { Keys.W, 'ㅉ' },
            { Keys.E, 'ㄸ' },
            { Keys.R, 'ㄲ' },
            { Keys.T, 'ㅆ' },
            { Keys.Y, 'ㅛ' },
            { Keys.U, 'ㅕ' },
            { Keys.I, 'ㅑ' },
            { Keys.O, 'ㅒ' },
            { Keys.P, 'ㅖ' },
            { Keys.A, 'ㅁ' },
            { Keys.S, 'ㄴ' },
            { Keys.D, 'ㅇ' },
            { Keys.F, 'ㄹ' },
            { Keys.G, 'ㅎ' },
            { Keys.H, 'ㅗ' },
            { Keys.J, 'ㅓ' },
            { Keys.K, 'ㅏ' },
            { Keys.L, 'ㅣ' },
            { Keys.Z, 'ㅋ' },
            { Keys.X, 'ㅌ' },
            { Keys.C, 'ㅊ' },
            { Keys.V, 'ㅍ' },
            { Keys.B, 'ㅠ' },
            { Keys.N, 'ㅜ' },
            { Keys.M, 'ㅡ' },
        };

        public static string jamoToHangul(string s초성, string s중성, string s종성) {
            int i초성위치, i중성위치, i종성위치;
            int iUniCode;
            i초성위치 = m초성Tbl.IndexOf(s초성);
            i중성위치 = m중성Tbl.IndexOf(s중성);
            i종성위치 = m종성Tbl.IndexOf(s종성);
            iUniCode = mUniCode한글Base + (i초성위치 * 21 + i중성위치) * 28 + i종성위치;
            char temp = Convert.ToChar(iUniCode);
            return temp.ToString();
        }

        #endregion

        public static List<char> charbuf = new List<char>();
        public static bool chatOpened = false;
        public static bool isHangul = false;
        public static bool isShift = false;
        public static int skips = 0;

        public static void SendText(string msg) {
            skips++;
            SendKeys.Send(msg);
        }

        public static IntPtr hookProc(int code, IntPtr wParam, IntPtr lParam) {
            IntPtr returnValue() {
                return CallNextHookEx(hhook, code, (int) wParam, lParam);
            }

            if (code >= 0) {
                if (wParam == (IntPtr) WM_KEYDOWN) {
                    if(paused) {
                        return returnValue();
                    }

                    if (skips > 0) {
                        skips--;
                        return returnValue();
                    }

                    int vkCode = Marshal.ReadInt32(lParam);
                    Keys key = (Keys) vkCode;

                    if (key == Keys.HangulMode && chatOpened) {
                        isHangul = !isHangul;
                        SetHangulEnabled(isHangul);
                        return (IntPtr) 1;
                    }

                    if ((ModifierKeys & Keys.Control) != 0 || (ModifierKeys & Keys.Alt) != 0)
                        return returnValue();

                    if (key == Keys.LShiftKey)
                        return returnValue();

                    if (key == Settings.Default.ChatKey && !chatOpened) {
                        chatOpened = true;
                        SetChatOpened(true);
                        return returnValue();
                    } else if (key == Keys.Escape && chatOpened) {
                        chatOpened = false;
                        isHangul = false;
                        SetChatOpened(false);
                        SetHangulEnabled(false);
                        charbuf.Clear();
                        
                        return returnValue();
                    } else if (key == Keys.Enter && chatOpened) {
                        chatOpened = false;
                        isHangul = false;
                        SetChatOpened(false);
                        SetHangulEnabled(false);
                        charbuf.Clear();
                        
                        return returnValue();
                    }

                    if (!chatOpened || !isHangul)
                        return returnValue();

                    isShift = (ModifierKeys & Keys.Shift) != 0;

                    if (key == Keys.Back) {
                        if (charbuf.Count == 0)
                            return returnValue();

                        charbuf.RemoveAt(charbuf.Count - 1);

                        if (charbuf.Count == 2) {
                            SendText("{BACKSPACE}");
                            SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                        } else if (charbuf.Count == 1) {
                            SendText("{BACKSPACE}");
                            SendText(charbuf[0].ToString());
                        } else if (charbuf.Count == 0) {
                            SendText("{BACKSPACE}");
                        }

                        return (IntPtr) 1;
                    }

                    if (hangulMap.ContainsKey(key)) {
                        char ch = isShift ? shiftHangulMap[key] : hangulMap[key];

                        charbuf.Add(ch);
                        AddLog($"Received Keypress: '{(isShift ? key.ToString() : key.ToString().ToLower())}' mapped as '{ch}'");

                        if (charbuf.Count == 1) {
                            SendText(charbuf[0].ToString());
                            if (!m초성Tbl.Contains(charbuf[0].ToString())) {
                                charbuf.Clear();
                                
                            }
                        } else if (charbuf.Count == 2) {
                            if (!m중성Tbl.Contains(charbuf[1].ToString())) {
                                SendText("{BACKSPACE}");
                                SendText(charbuf[0].ToString());
                                SendText(charbuf[1].ToString());
                                charbuf.Clear();
                                
                            } else {
                                SendText("{BACKSPACE}");
                                SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                            }
                        } else if (charbuf.Count == 3) {
                            if (m중성Tbl.Contains(charbuf[2].ToString())) {
                                string lastChar = charbuf[2].ToString();
                                charbuf.RemoveAt(2);

                                if (charbuf[1] == 'ㅗ') {
                                    if (lastChar == "ㅏ") {
                                        charbuf[1] = 'ㅘ';
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (lastChar == "ㅐ") {
                                        charbuf[1] = 'ㅙ';
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (lastChar == "ㅣ") {
                                        charbuf[1] = 'ㅚ';
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else {
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                        SendText(lastChar);
                                        charbuf.Clear();
                                        
                                    }
                                } else if (charbuf[1] == 'ㅜ') {
                                    if (lastChar == "ㅓ") {
                                        charbuf[1] = 'ㅝ';
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (lastChar == "ㅔ") {
                                        charbuf[1] = 'ㅞ';
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (lastChar == "ㅣ") {
                                        charbuf[1] = 'ㅟ';
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else {
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                        SendText(lastChar);
                                        charbuf.Clear();
                                        
                                    }
                                } else if (charbuf[1] == 'ㅡ' && lastChar == "ㅣ") {
                                    charbuf[1] = 'ㅢ';
                                    SendText("{BACKSPACE}");
                                    SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                } else {
                                    SendText(lastChar);
                                    charbuf.Clear();
                                    
                                }
                            } else if (m초성Tbl.Contains(charbuf[2].ToString())) {
                                SendText("{BACKSPACE}");
                                SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), charbuf[2].ToString()));
                            }
                        } else if (charbuf.Count == 4) {
                            if (m중성Tbl.Contains(charbuf[3].ToString())) {
                                if (mDbl종성Tbl.Contains(charbuf[2].ToString())) {
                                    if (charbuf[2] == 'ㄲ') {
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                        char 초성 = charbuf[2];
                                        char 중성 = charbuf[3];
                                        charbuf.Clear();

                                        charbuf.Add(초성);
                                        charbuf.Add(중성);
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (charbuf[2] == 'ㄳ') {
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), "ㄱ"));
                                        char 중성 = charbuf[3];
                                        charbuf.Clear();
                                        
                                        charbuf.Add('ㅅ');
                                        charbuf.Add(중성);
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (charbuf[2] == 'ㄵ') {
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), "ㄴ"));
                                        char 중성 = charbuf[3];
                                        charbuf.Clear();
                                        
                                        charbuf.Add('ㅈ');
                                        charbuf.Add(중성);
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (charbuf[2] == 'ㄶ') {
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), "ㄴ"));
                                        char 중성 = charbuf[3];
                                        charbuf.Clear();
                                        
                                        charbuf.Add('ㅎ');
                                        charbuf.Add(중성);
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (charbuf[2] == 'ㄺ') {
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), "ㄹ"));
                                        char 중성 = charbuf[3];
                                        charbuf.Clear();
                                        
                                        charbuf.Add('ㄱ');
                                        charbuf.Add(중성);
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (charbuf[2] == 'ㄻ') {
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), "ㄹ"));
                                        char 중성 = charbuf[3];
                                        charbuf.Clear();
                                        
                                        charbuf.Add('ㅁ');
                                        charbuf.Add(중성);
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (charbuf[2] == 'ㄼ') {
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), "ㄹ"));
                                        char 중성 = charbuf[3];
                                        charbuf.Clear();
                                        
                                        charbuf.Add('ㅂ');
                                        charbuf.Add(중성);
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (charbuf[2] == 'ㄽ') {
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), "ㄹ"));
                                        char 중성 = charbuf[3];
                                        charbuf.Clear();
                                        
                                        charbuf.Add('ㅅ');
                                        charbuf.Add(중성);
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (charbuf[2] == 'ㄾ') {
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), "ㄹ"));
                                        char 중성 = charbuf[3];
                                        charbuf.Clear();
                                        
                                        charbuf.Add('ㅌ');
                                        charbuf.Add(중성);
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (charbuf[2] == 'ㄿ') {
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), "ㄹ"));
                                        char 중성 = charbuf[3];
                                        charbuf.Clear();
                                        
                                        charbuf.Add('ㅍ');
                                        charbuf.Add(중성);
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (charbuf[2] == 'ㅀ') {
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), "ㄹ"));
                                        char 중성 = charbuf[3];
                                        charbuf.Clear();
                                        
                                        charbuf.Add('ㅎ');
                                        charbuf.Add(중성);
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    } else if (charbuf[2] == 'ㅄ') {
                                        SendText("{BACKSPACE}");
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), "ㅂ"));
                                        char 중성 = charbuf[3];
                                        charbuf.Clear();
                                        
                                        charbuf.Add('ㅅ');
                                        charbuf.Add(중성);
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    }
                                } else {
                                    SendText("{BACKSPACE}");
                                    SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                    char 초성 = charbuf[2];
                                    char 중성 = charbuf[3];
                                    charbuf.Clear();
                                    
                                    charbuf.Add(초성);
                                    charbuf.Add(중성);
                                    SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), ""));
                                }
                            } else {
                                if (charbuf[2] == 'ㄱ') {
                                    if (charbuf[3] == 'ㅅ') {
                                        SendText("{BACKSPACE}");
                                        charbuf.RemoveAt(3);
                                        charbuf[2] = 'ㄳ';
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), charbuf[2].ToString()));
                                        return (IntPtr) 1;
                                    }
                                } else if (charbuf[2] == 'ㄴ') {
                                    if (charbuf[3] == 'ㅈ') {
                                        SendText("{BACKSPACE}");
                                        charbuf.RemoveAt(3);
                                        charbuf[2] = 'ㄵ';
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), charbuf[2].ToString()));
                                        return (IntPtr) 1;
                                    } else if (charbuf[3] == 'ㅎ') {
                                        SendText("{BACKSPACE}");
                                        charbuf.RemoveAt(3);
                                        charbuf[2] = 'ㄶ';
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), charbuf[2].ToString()));
                                        return (IntPtr) 1;
                                    }
                                } else if (charbuf[2] == 'ㄹ') {
                                    if (charbuf[3] == 'ㅁ') {
                                        SendText("{BACKSPACE}");
                                        charbuf.RemoveAt(3);
                                        charbuf[2] = 'ㄻ';
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), charbuf[2].ToString()));
                                        return (IntPtr) 1;
                                    } else if (charbuf[3] == 'ㅂ') {
                                        SendText("{BACKSPACE}");
                                        charbuf.RemoveAt(3);
                                        charbuf[2] = 'ㄼ';
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), charbuf[2].ToString()));
                                        return (IntPtr) 1;
                                    } else if (charbuf[3] == 'ㅅ') {
                                        SendText("{BACKSPACE}");
                                        charbuf.RemoveAt(3);
                                        charbuf[2] = 'ㄽ';
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), charbuf[2].ToString()));
                                        return (IntPtr) 1;
                                    } else if (charbuf[3] == 'ㅌ') {
                                        SendText("{BACKSPACE}");
                                        charbuf.RemoveAt(3);
                                        charbuf[2] = 'ㄾ';
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), charbuf[2].ToString()));
                                        return (IntPtr) 1;
                                    } else if (charbuf[3] == 'ㅍ') {
                                        SendText("{BACKSPACE}");
                                        charbuf.RemoveAt(3);
                                        charbuf[2] = 'ㄿ';
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), charbuf[2].ToString()));
                                        return (IntPtr) 1;
                                    } else if (charbuf[3] == 'ㅎ') {
                                        SendText("{BACKSPACE}");
                                        charbuf.RemoveAt(3);
                                        charbuf[2] = 'ㅀ';
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), charbuf[2].ToString()));
                                        return (IntPtr) 1;
                                    } else if (charbuf[3] == 'ㄱ') {
                                        SendText("{BACKSPACE}");
                                        charbuf.RemoveAt(3);
                                        charbuf[2] = 'ㄺ';
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), charbuf[2].ToString()));
                                        return (IntPtr) 1;
                                    }
                                } else if (charbuf[2] == 'ㅂ') {
                                    if (charbuf[3] == 'ㅅ') {
                                        SendText("{BACKSPACE}");
                                        charbuf.RemoveAt(3);
                                        charbuf[2] = 'ㅄ';
                                        SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), charbuf[2].ToString()));
                                        return (IntPtr) 1;
                                    }
                                }

                                SendText("{BACKSPACE}");
                                SendText(jamoToHangul(charbuf[0].ToString(), charbuf[1].ToString(), charbuf[2].ToString()));
                                char 초성 = charbuf[3];
                                charbuf.Clear();
                                charbuf.Add(초성);
                                SendText(초성.ToString());
                            }
                        }

                        return (IntPtr) 1;
                    } else {
                        charbuf.Clear();

                        return CallNextHookEx(hhook, code, (int) wParam, lParam);
                    }
                }

                return CallNextHookEx(hhook, code, (int) wParam, lParam);
            }
            return CallNextHookEx(hhook, code, (int) wParam, lParam);
        }

        private void Form1_Load(object sender, EventArgs e) {
            instance = this;
            log = logBox;
            hangulEnabledLabel = HangulMode;
            chatOpenedLabel = ChatEnabled;
            logEnabled = LogEnabled;
            pause = Pause;

            List<Keys> keys = new List<Keys>(keyMap.Values);
            ChatKeySelect.SelectedIndex = keys.IndexOf(Settings.Default.ChatKey);

            SetLogEnabled(true);
            SetHook();
            AddLog("Successfully Started Keyboard Hook Process.");
            SetChatOpened(false);
            SetHangulEnabled(false);

            Updater.CheckUpdate();
        }

        private void Form1_FormClosing(object sender, FormClosingEventArgs e) {
            UnHook();
        }

        private void Form1_MouseDown(object sender, System.Windows.Forms.MouseEventArgs e) {
            if (e.Button == MouseButtons.Left) {
                ReleaseCapture();
                SendMessage(Handle, WM_NCLBUTTONDOWN, HT_CAPTION, 0);
            }
        }

        private void label_MouseDown(object sender, MouseEventArgs e) {
            if (e.Button == MouseButtons.Left) {
                ReleaseCapture();
                SendMessage(Handle, WM_NCLBUTTONDOWN, HT_CAPTION, 0);
            }
        }

        private void Form1_KeyDown(object sender, KeyEventArgs e) {
            if (e.KeyCode == Keys.Escape) {
                UnHook();
                Dispose();
            }
        }

        private void ExitButton_Click(object sender, EventArgs e) {
            UnHook();
            Dispose();
        }

        public static Form1 instance;
        public static Label hangulEnabledLabel;
        public static Label chatOpenedLabel;
        public static CheckBox logEnabled;
        public static RichTextBox log;
        public static Button pause;
        public static bool paused = false;
        public static void AddLog(string logMessage) {
            if (!logEnabled.Checked)
                return;
            log.Text += DateTime.Now.ToString("[dd/MM/yy hh:mm:ss tt]") + " [INFO] " + logMessage + Environment.NewLine;
            log.SelectionStart = log.TextLength;
            log.ScrollToCaret();
        }
        private static void SetChatOpened(bool value) {
            if (value) {
                chatOpenedLabel.Text = "채팅창 상태: 열림";
            } else {
                chatOpenedLabel.Text = "채팅창 상태: 닫힘";
            }
            if (!logEnabled.Checked)
                return;
            log.Text += DateTime.Now.ToString("[dd/MM/yy hh:mm:ss tt]") + " [INFO] Set ChatOpened to '" + value.ToString() + "'" + Environment.NewLine;
            log.SelectionStart = log.TextLength;
            log.ScrollToCaret();
        }
        private static void SetHangulEnabled(bool value) {
            if (value) {
                hangulEnabledLabel.Text = "한글 활성화: 활성화됨";
            } else {
                hangulEnabledLabel.Text = "채팅창 상태: 비활성화됨";
            }
            if (!logEnabled.Checked)
                return;
            log.Text += DateTime.Now.ToString("[dd/MM/yy hh:mm:ss tt]") + " [INFO] Set HangulEnabled to '" + value.ToString() + "'" + Environment.NewLine;
            log.SelectionStart = log.TextLength;
            log.ScrollToCaret();
        }
        private static void SetLogEnabled(bool value) {
            logEnabled.Checked = value;
        }

        private void Pause_Click(object sender, EventArgs e) {
            paused = !paused;
            if(paused) {
                pause.Text = "재시작";
            } else {
                pause.Text = "일시정지";
            }

            log.Text += DateTime.Now.ToString("[dd/MM/yy hh:mm:ss tt]") + " [INFO] " + (paused ? "Paused" : "Resumed") + " Keyboard Hook Process." + Environment.NewLine;
            log.SelectionStart = log.TextLength;
            log.ScrollToCaret();
        }
        private void LogEnabled_CheckedChanged(object sender, EventArgs e) {
            log.Text += DateTime.Now.ToString("[dd/MM/yy hh:mm:ss tt]") + " [INFO] Log " + (logEnabled.Checked ? "Enabled" : "Disabled") + Environment.NewLine;
            log.SelectionStart = log.TextLength;
            log.ScrollToCaret();
        }
        public static void Terminate() {
            UnHook();
            instance.Dispose();
        }

        private readonly Dictionary<string, Keys> keyMap = new Dictionary<string, Keys>() {
            { "Grave", (Keys) '`' },
            { "Tab", Keys.Tab },
            { "Enter", Keys.Enter },
            { "Q", Keys.Q },
            { "W", Keys.W },
            { "E", Keys.E },
            { "R", Keys.R },
            { "T", Keys.T },
            { "Y", Keys.Y },
            { "U", Keys.U },
            { "I", Keys.I },
            { "O", Keys.O },
            { "P", Keys.P },
            { "A", Keys.A },
            { "S", Keys.S },
            { "D", Keys.D },
            { "F", Keys.F },
            { "G", Keys.G },
            { "H", Keys.H },
            { "J", Keys.J },
            { "K", Keys.K },
            { "L", Keys.L },
            { "Z", Keys.Z },
            { "X", Keys.X },
            { "C", Keys.C },
            { "V", Keys.V },
            { "B", Keys.B },
            { "N", Keys.N },
            { "M", Keys.M },
            { "F1", Keys.F1 },
            { "F2", Keys.F2 },
            { "F3", Keys.F3 },
            { "F4", Keys.F4 },
            { "F5", Keys.F5 },
            { "F6", Keys.F6 },
            { "F7", Keys.F7 },
            { "F8", Keys.F8 },
            { "F9", Keys.F9 },
            { "F10", Keys.F10 },
            { "F11", Keys.F11 },
            { "F12", Keys.F12 }
        };

        private void ChatKeySelect_SelectedValueChanged(object sender, EventArgs e) {
            Keys key = keyMap[ChatKeySelect.SelectedItem.ToString()];
            Settings.Default.ChatKey = key;
            Settings.Default.Save();
            AddLog($"Set ChatOpen Key to {ChatKeySelect.SelectedItem}");
        }
    }
}
