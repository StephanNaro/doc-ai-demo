// SPDX-License-Identifier: GPL-3.0-or-later

namespace DocAiClient;

partial class MainForm
{
    /// <summary>
    ///  Required designer variable.
    /// </summary>
    private System.ComponentModel.IContainer components = null;

    public static Label lblCategory = new Label
    {
        Text = "Document Type:",
        Location = new System.Drawing.Point(20, 20),
        AutoSize = true
    };
    public static ComboBox cmbCategory = new ComboBox
    {
        Location = new System.Drawing.Point(20, 45),
        Size = new System.Drawing.Size(300, 25),
        DropDownStyle = ComboBoxStyle.DropDownList
    };
    public static Label lblQuery = new Label
    {
        Text = "Your question:",
        Location = new System.Drawing.Point(20, 85),
        AutoSize = true
    };
    public static TextBox txtQuery = new TextBox
    {
        Location = new System.Drawing.Point(20, 110),
        Size = new System.Drawing.Size(580, 100),
        Multiline = true,
        ScrollBars = ScrollBars.Vertical,
        AcceptsReturn = true
    };
    public static Button btnAsk = new Button
    {
        Text = "Ask",
        Location = new System.Drawing.Point(20, 220),
        Size = new System.Drawing.Size(120, 40),
        BackColor = System.Drawing.Color.FromArgb(0, 102, 204),
        ForeColor = System.Drawing.Color.White,
        FlatStyle = FlatStyle.Flat
    };
    public static TextBox txtResult = new TextBox
    {
        Location = new System.Drawing.Point(20, 270),
        Size = new System.Drawing.Size(580, 220),
        Multiline = true,
        ScrollBars = ScrollBars.Vertical,
        ReadOnly = true,
        BackColor = System.Drawing.Color.White,
        Font = new System.Drawing.Font("Consolas", 9.75f)
    };

    /// <summary>
    ///  Clean up any resources being used.
    /// </summary>
    /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
    protected override void Dispose(bool disposing)
    {
        if (disposing && (components != null))
        {
            components.Dispose();
        }
        base.Dispose(disposing);
    }

    /// <summary>
    ///  Required method for Designer support - do not modify
    ///  the contents of this method with the code editor.
    /// </summary>
    private void InitializeComponent()
    {
        components = new System.ComponentModel.Container();
        AutoScaleMode = AutoScaleMode.Font;
        ClientSize = new Size(800, 450);

        this.Text = "Doc AI Client";
        this.Size = new System.Drawing.Size(650, 550);

        cmbCategory.Items.AddRange(new[]
        {
            "Invoices",
            "Employment Contracts",
            "Customer Support",
            "Knowledge Base"
        });
        cmbCategory.SelectedIndex = 0; // default to Invoices

        this.Controls.Add(lblQuery);
        this.Controls.Add(txtQuery);
        this.Controls.Add(btnAsk);
        this.Controls.Add(txtResult);
        this.Controls.Add(lblCategory);
        this.Controls.Add(cmbCategory);
    }
}
