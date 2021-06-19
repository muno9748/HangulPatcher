
namespace HangulCraft {
    partial class UpdateAlert {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing) {
            if (disposing && (components != null)) {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent() {
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(UpdateAlert));
            this.Exit = new System.Windows.Forms.Button();
            this.pictureBox1 = new System.Windows.Forms.PictureBox();
            this.label1 = new System.Windows.Forms.Label();
            this.label2 = new System.Windows.Forms.Label();
            this.downloadLink = new System.Windows.Forms.LinkLabel();
            this.label3 = new System.Windows.Forms.Label();
            this.panel1 = new System.Windows.Forms.Panel();
            ((System.ComponentModel.ISupportInitialize)(this.pictureBox1)).BeginInit();
            this.panel1.SuspendLayout();
            this.SuspendLayout();
            // 
            // Exit
            // 
            this.Exit.Font = new System.Drawing.Font(FontLibrary.Families[0], 9.75F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(129)));
            this.Exit.Location = new System.Drawing.Point(470, 129);
            this.Exit.Name = "Exit";
            this.Exit.Size = new System.Drawing.Size(118, 23);
            this.Exit.TabIndex = 0;
            this.Exit.Text = "종료";
            this.Exit.UseVisualStyleBackColor = true;
            this.Exit.Click += new System.EventHandler(this.Exit_Click);
            // 
            // pictureBox1
            // 
            this.pictureBox1.Image = ((System.Drawing.Image)(resources.GetObject("pictureBox1.Image")));
            this.pictureBox1.Location = new System.Drawing.Point(470, 8);
            this.pictureBox1.Name = "pictureBox1";
            this.pictureBox1.Size = new System.Drawing.Size(118, 115);
            this.pictureBox1.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pictureBox1.TabIndex = 9;
            this.pictureBox1.TabStop = false;
            this.pictureBox1.MouseDown += new System.Windows.Forms.MouseEventHandler(this.MoveWindow);
            // 
            // label1
            // 
            this.label1.AutoSize = true;
            this.label1.Font = new System.Drawing.Font(FontLibrary.Families[0], 30.25F);
            this.label1.Location = new System.Drawing.Point(3, 13);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(352, 47);
            this.label1.TabIndex = 10;
            this.label1.Text = "오래된 버전입니다.";
            this.label1.MouseDown += new System.Windows.Forms.MouseEventHandler(this.MoveWindow);
            // 
            // label2
            // 
            this.label2.AutoSize = true;
            this.label2.Font = new System.Drawing.Font(FontLibrary.Families[0], 10.25F);
            this.label2.Location = new System.Drawing.Point(8, 77);
            this.label2.Name = "label2";
            this.label2.Size = new System.Drawing.Size(218, 32);
            this.label2.TabIndex = 11;
            this.label2.Text = "해당 버전은 사용이 중지되었습니다.\r\n업데이트가 필요합니다.";
            this.label2.MouseDown += new System.Windows.Forms.MouseEventHandler(this.MoveWindow);
            // 
            // downloadLink
            // 
            this.downloadLink.ActiveLinkColor = System.Drawing.Color.FromArgb(((int)(((byte)(50)))), ((int)(((byte)(0)))), ((int)(((byte)(112)))));
            this.downloadLink.AutoSize = true;
            this.downloadLink.Font = new System.Drawing.Font(FontLibrary.Families[0], 9.75F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(129)));
            this.downloadLink.LinkColor = System.Drawing.SystemColors.Highlight;
            this.downloadLink.Location = new System.Drawing.Point(9, 111);
            this.downloadLink.Name = "downloadLink";
            this.downloadLink.RightToLeft = System.Windows.Forms.RightToLeft.No;
            this.downloadLink.Size = new System.Drawing.Size(89, 15);
            this.downloadLink.TabIndex = 12;
            this.downloadLink.TabStop = true;
            this.downloadLink.Text = "다운로드 링크";
            this.downloadLink.VisitedLinkColor = System.Drawing.SystemColors.Highlight;
            this.downloadLink.LinkClicked += new System.Windows.Forms.LinkLabelLinkClickedEventHandler(this.downloadLink_LinkClicked);
            // 
            // label3
            // 
            this.label3.AutoSize = true;
            this.label3.Font = new System.Drawing.Font(FontLibrary.Families[0], 10.25F);
            this.label3.Location = new System.Drawing.Point(97, 111);
            this.label3.Name = "label3";
            this.label3.Size = new System.Drawing.Size(196, 16);
            this.label3.TabIndex = 13;
            this.label3.Text = "에서 새 버전을 다운받아 주세요.";
            this.label3.MouseDown += new System.Windows.Forms.MouseEventHandler(this.MoveWindow);
            // 
            // panel1
            // 
            this.panel1.Controls.Add(this.label1);
            this.panel1.Controls.Add(this.label3);
            this.panel1.Controls.Add(this.Exit);
            this.panel1.Controls.Add(this.downloadLink);
            this.panel1.Controls.Add(this.pictureBox1);
            this.panel1.Controls.Add(this.label2);
            this.panel1.Location = new System.Drawing.Point(0, 1);
            this.panel1.Name = "panel1";
            this.panel1.Size = new System.Drawing.Size(597, 158);
            this.panel1.TabIndex = 14;
            this.panel1.MouseDown += new System.Windows.Forms.MouseEventHandler(this.MoveWindow);
            // 
            // UpdateAlert
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(7F, 12F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.SystemColors.Control;
            this.ClientSize = new System.Drawing.Size(598, 158);
            this.Controls.Add(this.panel1);
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.MaximizeBox = false;
            this.Name = "UpdateAlert";
            this.Text = "UpdateAlert";
            ((System.ComponentModel.ISupportInitialize)(this.pictureBox1)).EndInit();
            this.panel1.ResumeLayout(false);
            this.panel1.PerformLayout();
            this.ResumeLayout(false);

        }

        #endregion

        private System.Windows.Forms.Button Exit;
        private System.Windows.Forms.PictureBox pictureBox1;
        private System.Windows.Forms.Label label1;
        private System.Windows.Forms.Label label2;
        private System.Windows.Forms.LinkLabel downloadLink;
        private System.Windows.Forms.Label label3;
        private System.Windows.Forms.Panel panel1;
    }
}