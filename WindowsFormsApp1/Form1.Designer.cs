
namespace HangulCraft
{
    partial class Form1
    {
        /// <summary>
        /// 필수 디자이너 변수입니다.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// 사용 중인 모든 리소스를 정리합니다.
        /// </summary>
        /// <param name="disposing">관리되는 리소스를 삭제해야 하면 true이고, 그렇지 않으면 false입니다.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form 디자이너에서 생성한 코드

        /// <summary>
        /// 디자이너 지원에 필요한 메서드입니다. 
        /// 이 메서드의 내용을 코드 편집기로 수정하지 마세요.
        /// </summary>
        private void InitializeComponent()
        {
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(Form1));
            this.label = new System.Windows.Forms.Label();
            this.ExitButton = new System.Windows.Forms.Button();
            this.HangulMode = new System.Windows.Forms.Label();
            this.ChatEnabled = new System.Windows.Forms.Label();
            this.logBox = new System.Windows.Forms.RichTextBox();
            this.label1 = new System.Windows.Forms.Label();
            this.label3 = new System.Windows.Forms.Label();
            this.pictureBox1 = new System.Windows.Forms.PictureBox();
            this.LogEnabled = new System.Windows.Forms.CheckBox();
            this.Pause = new System.Windows.Forms.Button();
            this.label2 = new System.Windows.Forms.Label();
            this.captureButton = new System.Windows.Forms.Button();
            this.button1 = new System.Windows.Forms.Button();
            this.button2 = new System.Windows.Forms.Button();
            this.button3 = new System.Windows.Forms.Button();
            this.label4 = new System.Windows.Forms.Label();
            ((System.ComponentModel.ISupportInitialize)(this.pictureBox1)).BeginInit();
            this.SuspendLayout();
            // 
            // label
            // 
            this.label.AutoSize = true;
            this.label.Location = new System.Drawing.Point(4, 9);
            this.label.Name = "label";
            this.label.Size = new System.Drawing.Size(157, 12);
            this.label.TabIndex = 0;
            this.label.Text = "키보드 입력을 캡쳐중입니다";
            this.label.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            this.label.MouseDown += new System.Windows.Forms.MouseEventHandler(this.label_MouseDown);
            // 
            // ExitButton
            // 
            this.ExitButton.Location = new System.Drawing.Point(564, 510);
            this.ExitButton.Name = "ExitButton";
            this.ExitButton.Size = new System.Drawing.Size(150, 62);
            this.ExitButton.TabIndex = 1;
            this.ExitButton.Text = "종료";
            this.ExitButton.UseVisualStyleBackColor = true;
            this.ExitButton.Click += new System.EventHandler(this.ExitButton_Click);
            // 
            // HangulMode
            // 
            this.HangulMode.AutoSize = true;
            this.HangulMode.Location = new System.Drawing.Point(8, 79);
            this.HangulMode.Name = "HangulMode";
            this.HangulMode.Size = new System.Drawing.Size(137, 12);
            this.HangulMode.TabIndex = 2;
            this.HangulMode.Text = "한글 활성화: 비활성화됨";
            this.HangulMode.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // ChatEnabled
            // 
            this.ChatEnabled.AutoSize = true;
            this.ChatEnabled.Location = new System.Drawing.Point(8, 99);
            this.ChatEnabled.Name = "ChatEnabled";
            this.ChatEnabled.Size = new System.Drawing.Size(101, 12);
            this.ChatEnabled.TabIndex = 3;
            this.ChatEnabled.Text = "채팅창 상태: 닫힘";
            this.ChatEnabled.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // logBox
            // 
            this.logBox.Font = new System.Drawing.Font("Consolas", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.logBox.Location = new System.Drawing.Point(12, 155);
            this.logBox.Name = "logBox";
            this.logBox.ReadOnly = true;
            this.logBox.Size = new System.Drawing.Size(702, 349);
            this.logBox.TabIndex = 5;
            this.logBox.Text = "";
            // 
            // label1
            // 
            this.label1.AutoSize = true;
            this.label1.Location = new System.Drawing.Point(9, 558);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(258, 12);
            this.label1.TabIndex = 6;
            this.label1.Text = "한영 전환 키: 한/영 (Ctrl가 아닌 키보드 한/영)\r\n";
            this.label1.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // label3
            // 
            this.label3.AutoSize = true;
            this.label3.ForeColor = System.Drawing.Color.Gray;
            this.label3.Location = new System.Drawing.Point(9, 535);
            this.label3.Name = "label3";
            this.label3.Size = new System.Drawing.Size(118, 12);
            this.label3.TabIndex = 7;
            this.label3.Text = "© Made by ilyMuno";
            this.label3.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // pictureBox1
            // 
            this.pictureBox1.Image = ((System.Drawing.Image)(resources.GetObject("pictureBox1.Image")));
            this.pictureBox1.Location = new System.Drawing.Point(596, 9);
            this.pictureBox1.Name = "pictureBox1";
            this.pictureBox1.Size = new System.Drawing.Size(118, 115);
            this.pictureBox1.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pictureBox1.TabIndex = 8;
            this.pictureBox1.TabStop = false;
            // 
            // LogEnabled
            // 
            this.LogEnabled.AutoSize = true;
            this.LogEnabled.Location = new System.Drawing.Point(14, 134);
            this.LogEnabled.Name = "LogEnabled";
            this.LogEnabled.Size = new System.Drawing.Size(45, 16);
            this.LogEnabled.TabIndex = 9;
            this.LogEnabled.Text = "Log";
            this.LogEnabled.UseVisualStyleBackColor = true;
            this.LogEnabled.CheckedChanged += new System.EventHandler(this.LogEnabled_CheckedChanged);
            // 
            // Pause
            // 
            this.Pause.Location = new System.Drawing.Point(408, 510);
            this.Pause.Name = "Pause";
            this.Pause.Size = new System.Drawing.Size(150, 62);
            this.Pause.TabIndex = 10;
            this.Pause.Text = "일시중지";
            this.Pause.UseVisualStyleBackColor = true;
            this.Pause.Click += new System.EventHandler(this.Pause_Click);
            // 
            // label2
            // 
            this.label2.AutoSize = true;
            this.label2.Location = new System.Drawing.Point(8, 55);
            this.label2.Name = "label2";
            this.label2.Size = new System.Drawing.Size(101, 12);
            this.label2.TabIndex = 12;
            this.label2.Text = "채팅 오픈키 설정:";
            this.label2.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // captureButton
            // 
            this.captureButton.Location = new System.Drawing.Point(115, 50);
            this.captureButton.Name = "captureButton";
            this.captureButton.Size = new System.Drawing.Size(217, 23);
            this.captureButton.TabIndex = 13;
            this.captureButton.Text = "BIND: T";
            this.captureButton.UseVisualStyleBackColor = true;
            this.captureButton.Click += new System.EventHandler(this.button1_Click);
            this.captureButton.KeyDown += new System.Windows.Forms.KeyEventHandler(this.captureButton_KeyDown);
            // 
            // button1
            // 
            this.button1.Location = new System.Drawing.Point(338, 50);
            this.button1.Name = "button1";
            this.button1.Size = new System.Drawing.Size(119, 23);
            this.button1.TabIndex = 14;
            this.button1.Text = "CTRL";
            this.button1.UseVisualStyleBackColor = true;
            this.button1.Click += new System.EventHandler(this.button1_Click_1);
            // 
            // button2
            // 
            this.button2.Location = new System.Drawing.Point(463, 50);
            this.button2.Name = "button2";
            this.button2.Size = new System.Drawing.Size(127, 23);
            this.button2.TabIndex = 15;
            this.button2.Text = "ALT";
            this.button2.UseVisualStyleBackColor = true;
            this.button2.Click += new System.EventHandler(this.button2_Click);
            // 
            // button3
            // 
            this.button3.Location = new System.Drawing.Point(338, 74);
            this.button3.Name = "button3";
            this.button3.Size = new System.Drawing.Size(119, 23);
            this.button3.TabIndex = 16;
            this.button3.Text = "RCTRL";
            this.button3.UseVisualStyleBackColor = true;
            this.button3.Click += new System.EventHandler(this.button3_Click);
            // 
            // label4
            // 
            this.label4.AutoSize = true;
            this.label4.Location = new System.Drawing.Point(4, 35);
            this.label4.Name = "label4";
            this.label4.Size = new System.Drawing.Size(500, 12);
            this.label4.TabIndex = 17;
            this.label4.Text = "키보드 입력 받기중 CTRL, ALT, RCTRL 버튼은 불안정하니 되도록이면 사용을 금해주세요.";
            // 
            // Form1
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(7F, 12F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(738, 583);
            this.Controls.Add(this.label4);
            this.Controls.Add(this.button3);
            this.Controls.Add(this.button2);
            this.Controls.Add(this.button1);
            this.Controls.Add(this.captureButton);
            this.Controls.Add(this.label2);
            this.Controls.Add(this.Pause);
            this.Controls.Add(this.LogEnabled);
            this.Controls.Add(this.pictureBox1);
            this.Controls.Add(this.label3);
            this.Controls.Add(this.label1);
            this.Controls.Add(this.logBox);
            this.Controls.Add(this.ChatEnabled);
            this.Controls.Add(this.HangulMode);
            this.Controls.Add(this.ExitButton);
            this.Controls.Add(this.label);
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.Name = "Form1";
            this.Text = "Form1";
            this.FormClosing += new System.Windows.Forms.FormClosingEventHandler(this.Form1_FormClosing);
            this.Load += new System.EventHandler(this.Form1_Load);
            this.KeyDown += new System.Windows.Forms.KeyEventHandler(this.Form1_KeyDown);
            this.MouseDown += new System.Windows.Forms.MouseEventHandler(this.Form1_MouseDown);
            ((System.ComponentModel.ISupportInitialize)(this.pictureBox1)).EndInit();
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.Label label;
        private System.Windows.Forms.Button ExitButton;
        private System.Windows.Forms.Label HangulMode;
        private System.Windows.Forms.Label ChatEnabled;
        private System.Windows.Forms.RichTextBox logBox;
        private System.Windows.Forms.Label label1;
        private System.Windows.Forms.Label label3;
        private System.Windows.Forms.PictureBox pictureBox1;
        private System.Windows.Forms.CheckBox LogEnabled;
        private System.Windows.Forms.Button Pause;
        private System.Windows.Forms.Label label2;
        private System.Windows.Forms.Button captureButton;
        private System.Windows.Forms.Button button1;
        private System.Windows.Forms.Button button2;
        private System.Windows.Forms.Button button3;
        private System.Windows.Forms.Label label4;
    }
}

