
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
            ((System.ComponentModel.ISupportInitialize)(this.pictureBox1)).BeginInit();
            this.SuspendLayout();
            // 
            // label
            // 
            this.label.AutoSize = true;
            this.label.Font = new System.Drawing.Font(FontLibrary.Families[0], 30.25F);
            this.label.Location = new System.Drawing.Point(4, 9);
            this.label.Name = "label";
            this.label.Size = new System.Drawing.Size(510, 47);
            this.label.TabIndex = 0;
            this.label.Text = "키보드 입력을 캡쳐중입니다";
            this.label.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            this.label.MouseDown += new System.Windows.Forms.MouseEventHandler(this.label_MouseDown);
            // 
            // ExitButton
            // 
            this.ExitButton.Font = new System.Drawing.Font(FontLibrary.Families[0], 18F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(129)));
            this.ExitButton.Location = new System.Drawing.Point(564, 485);
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
            this.HangulMode.Font = new System.Drawing.Font(FontLibrary.Families[0], 12.25F);
            this.HangulMode.Location = new System.Drawing.Point(8, 56);
            this.HangulMode.Name = "HangulMode";
            this.HangulMode.Size = new System.Drawing.Size(183, 20);
            this.HangulMode.TabIndex = 2;
            this.HangulMode.Text = "한글 활성화: 비활성화됨";
            this.HangulMode.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // ChatEnabled
            // 
            this.ChatEnabled.AutoSize = true;
            this.ChatEnabled.Font = new System.Drawing.Font(FontLibrary.Families[0], 12.25F);
            this.ChatEnabled.Location = new System.Drawing.Point(8, 76);
            this.ChatEnabled.Name = "ChatEnabled";
            this.ChatEnabled.Size = new System.Drawing.Size(135, 20);
            this.ChatEnabled.TabIndex = 3;
            this.ChatEnabled.Text = "채팅창 상태: 닫힘";
            this.ChatEnabled.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // logBox
            // 
            this.logBox.Font = new System.Drawing.Font("Consolas", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.logBox.Location = new System.Drawing.Point(12, 130);
            this.logBox.Name = "logBox";
            this.logBox.ReadOnly = true;
            this.logBox.Size = new System.Drawing.Size(702, 349);
            this.logBox.TabIndex = 5;
            this.logBox.Text = "";
            // 
            // label1
            // 
            this.label1.AutoSize = true;
            this.label1.Font = new System.Drawing.Font(FontLibrary.Families[0], 11.25F);
            this.label1.Location = new System.Drawing.Point(9, 533);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(303, 17);
            this.label1.TabIndex = 6;
            this.label1.Text = "한영 전환 키: 한/영 (Ctrl가 아닌 키보드 한/영)\r\n";
            this.label1.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // label3
            // 
            this.label3.AutoSize = true;
            this.label3.Font = new System.Drawing.Font(FontLibrary.Families[0], 11.25F);
            this.label3.ForeColor = System.Drawing.Color.Gray;
            this.label3.Location = new System.Drawing.Point(9, 510);
            this.label3.Name = "label3";
            this.label3.Size = new System.Drawing.Size(157, 17);
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
            this.LogEnabled.Font = new System.Drawing.Font(FontLibrary.Families[0], 8.999999F);
            this.LogEnabled.Location = new System.Drawing.Point(14, 112);
            this.LogEnabled.Name = "LogEnabled";
            this.LogEnabled.Size = new System.Drawing.Size(51, 18);
            this.LogEnabled.TabIndex = 9;
            this.LogEnabled.Text = "Log";
            this.LogEnabled.UseVisualStyleBackColor = true;
            this.LogEnabled.CheckedChanged += new System.EventHandler(this.LogEnabled_CheckedChanged);
            // 
            // Pause
            // 
            this.Pause.Font = new System.Drawing.Font(FontLibrary.Families[0], 18F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(129)));
            this.Pause.Location = new System.Drawing.Point(408, 485);
            this.Pause.Name = "Pause";
            this.Pause.Size = new System.Drawing.Size(150, 62);
            this.Pause.TabIndex = 10;
            this.Pause.Text = "일시중지";
            this.Pause.UseVisualStyleBackColor = true;
            this.Pause.Click += new System.EventHandler(this.Pause_Click);
            // 
            // Form1
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(7F, 12F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(738, 559);
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
    }
}

