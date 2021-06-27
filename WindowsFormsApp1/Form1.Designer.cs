
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
            this.label3 = new System.Windows.Forms.Label();
            this.pictureBox1 = new System.Windows.Forms.PictureBox();
            this.LogEnabled = new System.Windows.Forms.CheckBox();
            this.Pause = new System.Windows.Forms.Button();
            this.ChatOpenKeyInfo = new System.Windows.Forms.Label();
            this.captureButton = new System.Windows.Forms.Button();
            this.groupBox1 = new System.Windows.Forms.GroupBox();
            this.label1 = new System.Windows.Forms.Label();
            this.label5 = new System.Windows.Forms.Label();
            this.label6 = new System.Windows.Forms.Label();
            this.groupBox2 = new System.Windows.Forms.GroupBox();
            this.comboBox1 = new System.Windows.Forms.ComboBox();
            this.label2 = new System.Windows.Forms.Label();
            ((System.ComponentModel.ISupportInitialize)(this.pictureBox1)).BeginInit();
            this.groupBox1.SuspendLayout();
            this.groupBox2.SuspendLayout();
            this.SuspendLayout();
            // 
            // label
            // 
            this.label.AutoSize = true;
            this.label.Font = new System.Drawing.Font(FontLibrary.globalFont, 35F);
            this.label.Location = new System.Drawing.Point(4, 9);
            this.label.Name = "label";
            this.label.Size = new System.Drawing.Size(589, 54);
            this.label.TabIndex = 0;
            this.label.Text = "키보드 입력을 캡쳐중입니다";
            this.label.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            this.label.MouseDown += new System.Windows.Forms.MouseEventHandler(this.label_MouseDown);
            // 
            // ExitButton
            // 
            this.ExitButton.Font = new System.Drawing.Font(FontLibrary.globalFont, 18F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
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
            this.HangulMode.Font = new System.Drawing.Font(FontLibrary.globalFont, 10F);
            this.HangulMode.Location = new System.Drawing.Point(353, 300);
            this.HangulMode.Name = "HangulMode";
            this.HangulMode.Size = new System.Drawing.Size(149, 16);
            this.HangulMode.TabIndex = 2;
            this.HangulMode.Text = "한글 활성화: 비활성화됨";
            this.HangulMode.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // ChatEnabled
            // 
            this.ChatEnabled.AutoSize = true;
            this.ChatEnabled.Font = new System.Drawing.Font(FontLibrary.globalFont, 10F);
            this.ChatEnabled.Location = new System.Drawing.Point(353, 316);
            this.ChatEnabled.Name = "ChatEnabled";
            this.ChatEnabled.Size = new System.Drawing.Size(110, 16);
            this.ChatEnabled.TabIndex = 3;
            this.ChatEnabled.Text = "채팅창 상태: 닫힘";
            this.ChatEnabled.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // logBox
            // 
            this.logBox.Font = new System.Drawing.Font("Consolas", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.logBox.Location = new System.Drawing.Point(12, 105);
            this.logBox.Name = "logBox";
            this.logBox.ReadOnly = true;
            this.logBox.Size = new System.Drawing.Size(702, 349);
            this.logBox.TabIndex = 5;
            this.logBox.Text = "";
            // 
            // label3
            // 
            this.label3.AutoSize = true;
            this.label3.Font = new System.Drawing.Font(FontLibrary.globalFont, 15F);
            this.label3.ForeColor = System.Drawing.Color.Gray;
            this.label3.Location = new System.Drawing.Point(737, 549);
            this.label3.Name = "label3";
            this.label3.Size = new System.Drawing.Size(210, 23);
            this.label3.TabIndex = 7;
            this.label3.Text = "© Made by ilyMuno";
            this.label3.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // pictureBox1
            // 
            this.pictureBox1.Image = ((System.Drawing.Image)(resources.GetObject("pictureBox1.Image")));
            this.pictureBox1.Location = new System.Drawing.Point(619, 10);
            this.pictureBox1.Name = "pictureBox1";
            this.pictureBox1.Size = new System.Drawing.Size(95, 92);
            this.pictureBox1.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pictureBox1.TabIndex = 8;
            this.pictureBox1.TabStop = false;
            // 
            // LogEnabled
            // 
            this.LogEnabled.AutoSize = true;
            this.LogEnabled.Font = new System.Drawing.Font(FontLibrary.globalFont, 9F);
            this.LogEnabled.Location = new System.Drawing.Point(14, 84);
            this.LogEnabled.Name = "LogEnabled";
            this.LogEnabled.Size = new System.Drawing.Size(51, 18);
            this.LogEnabled.TabIndex = 9;
            this.LogEnabled.Text = "Log";
            this.LogEnabled.UseVisualStyleBackColor = true;
            this.LogEnabled.CheckedChanged += new System.EventHandler(this.LogEnabled_CheckedChanged);
            // 
            // Pause
            // 
            this.Pause.Font = new System.Drawing.Font(FontLibrary.globalFont, 18F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.Pause.Location = new System.Drawing.Point(408, 510);
            this.Pause.Name = "Pause";
            this.Pause.Size = new System.Drawing.Size(150, 62);
            this.Pause.TabIndex = 10;
            this.Pause.Text = "일시중지";
            this.Pause.UseVisualStyleBackColor = true;
            this.Pause.Click += new System.EventHandler(this.Pause_Click);
            // 
            // ChatOpenKeyInfo
            // 
            this.ChatOpenKeyInfo.AutoSize = true;
            this.ChatOpenKeyInfo.Font = new System.Drawing.Font(FontLibrary.globalFont, 10F);
            this.ChatOpenKeyInfo.Location = new System.Drawing.Point(6, 19);
            this.ChatOpenKeyInfo.Name = "ChatOpenKeyInfo";
            this.ChatOpenKeyInfo.Size = new System.Drawing.Size(97, 16);
            this.ChatOpenKeyInfo.TabIndex = 12;
            this.ChatOpenKeyInfo.Text = "현재 채팅창 키:";
            this.ChatOpenKeyInfo.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // captureButton
            // 
            this.captureButton.Location = new System.Drawing.Point(9, 37);
            this.captureButton.Name = "captureButton";
            this.captureButton.Size = new System.Drawing.Size(269, 23);
            this.captureButton.TabIndex = 13;
            this.captureButton.Text = "키 설정하기";
            this.captureButton.UseVisualStyleBackColor = true;
            this.captureButton.Click += new System.EventHandler(this.button1_Click);
            this.captureButton.KeyDown += new System.Windows.Forms.KeyEventHandler(this.captureButton_KeyDown);
            // 
            // groupBox1
            // 
            this.groupBox1.Controls.Add(this.captureButton);
            this.groupBox1.Controls.Add(this.ChatOpenKeyInfo);
            this.groupBox1.Font = new System.Drawing.Font(FontLibrary.globalFont, 9F);
            this.groupBox1.Location = new System.Drawing.Point(732, 68);
            this.groupBox1.Name = "groupBox1";
            this.groupBox1.Size = new System.Drawing.Size(284, 72);
            this.groupBox1.TabIndex = 18;
            this.groupBox1.TabStop = false;
            this.groupBox1.Text = "채팅 오픈키 설정";
            // 
            // label1
            // 
            this.label1.BorderStyle = System.Windows.Forms.BorderStyle.Fixed3D;
            this.label1.Location = new System.Drawing.Point(724, 7);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(2, 570);
            this.label1.TabIndex = 19;
            // 
            // label5
            // 
            this.label5.AutoSize = true;
            this.label5.Font = new System.Drawing.Font(FontLibrary.globalFont, 30F);
            this.label5.Location = new System.Drawing.Point(733, 8);
            this.label5.Name = "label5";
            this.label5.Size = new System.Drawing.Size(193, 46);
            this.label5.TabIndex = 20;
            this.label5.Text = "Settings";
            this.label5.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // label6
            // 
            this.label6.BorderStyle = System.Windows.Forms.BorderStyle.Fixed3D;
            this.label6.Location = new System.Drawing.Point(730, 58);
            this.label6.Name = "label6";
            this.label6.Size = new System.Drawing.Size(288, 2);
            this.label6.TabIndex = 21;
            // 
            // groupBox2
            // 
            this.groupBox2.Controls.Add(this.comboBox1);
            this.groupBox2.Controls.Add(this.label2);
            this.groupBox2.Font = new System.Drawing.Font(FontLibrary.globalFont, 9F);
            this.groupBox2.Location = new System.Drawing.Point(732, 146);
            this.groupBox2.Name = "groupBox2";
            this.groupBox2.Size = new System.Drawing.Size(284, 47);
            this.groupBox2.TabIndex = 19;
            this.groupBox2.TabStop = false;
            this.groupBox2.Text = "한/영 전환키 설정";
            // 
            // comboBox1
            // 
            this.comboBox1.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
            this.comboBox1.Font = new System.Drawing.Font(FontLibrary.globalFont, 8F);
            this.comboBox1.FormattingEnabled = true;
            this.comboBox1.Items.AddRange(new object[] {
            "한/영",
            "Control",
            "Alt"});
            this.comboBox1.Location = new System.Drawing.Point(113, 16);
            this.comboBox1.Name = "comboBox1";
            this.comboBox1.Size = new System.Drawing.Size(165, 21);
            this.comboBox1.TabIndex = 13;
            this.comboBox1.SelectedValueChanged += new System.EventHandler(this.comboBox1_SelectedValueChanged);
            // 
            // label2
            // 
            this.label2.AutoSize = true;
            this.label2.Font = new System.Drawing.Font(FontLibrary.globalFont, 11F);
            this.label2.Location = new System.Drawing.Point(6, 20);
            this.label2.Name = "label2";
            this.label2.Size = new System.Drawing.Size(103, 17);
            this.label2.TabIndex = 12;
            this.label2.Text = "현재 채팅창 키:";
            this.label2.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // Form1
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(7F, 12F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(1024, 583);
            this.Controls.Add(this.groupBox2);
            this.Controls.Add(this.label6);
            this.Controls.Add(this.label5);
            this.Controls.Add(this.label1);
            this.Controls.Add(this.groupBox1);
            this.Controls.Add(this.Pause);
            this.Controls.Add(this.LogEnabled);
            this.Controls.Add(this.pictureBox1);
            this.Controls.Add(this.label3);
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
            this.groupBox1.ResumeLayout(false);
            this.groupBox1.PerformLayout();
            this.groupBox2.ResumeLayout(false);
            this.groupBox2.PerformLayout();
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.Label label;
        private System.Windows.Forms.Button ExitButton;
        private System.Windows.Forms.Label HangulMode;
        private System.Windows.Forms.Label ChatEnabled;
        private System.Windows.Forms.RichTextBox logBox;
        private System.Windows.Forms.Label label3;
        private System.Windows.Forms.PictureBox pictureBox1;
        private System.Windows.Forms.CheckBox LogEnabled;
        private System.Windows.Forms.Button Pause;
        private System.Windows.Forms.Label ChatOpenKeyInfo;
        private System.Windows.Forms.Button captureButton;
        private System.Windows.Forms.GroupBox groupBox1;
        private System.Windows.Forms.Label label1;
        private System.Windows.Forms.Label label5;
        private System.Windows.Forms.Label label6;
        private System.Windows.Forms.GroupBox groupBox2;
        private System.Windows.Forms.ComboBox comboBox1;
        private System.Windows.Forms.Label label2;
    }
}

