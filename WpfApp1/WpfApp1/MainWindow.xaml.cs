using System;
using System.Collections.Generic;
using System.Data;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;
using MySql.Data;
using MySql.Data.MySqlClient;
using ExcelLibrary;



namespace WpfApp1
{
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();
            Simple_Button.Click += (s, e) => FillGrid();
            Simple_Button_Copy.Click += (s, e) => writexl();
        }

        private string server;
        private string database;
        private string user;
        private string password;
        private string port;
        private string connectionString;

        public void FillGrid()
        {
            server = "localhost";
            database = "world";
            user = "root";
            password = "v?557ZX";
            port = "3306";

            connectionString = $"server={server};port={port};user id={user}; password={password}; database={database};";

            using (MySqlConnection connection = new MySqlConnection(connectionString))
            {
                connection.Open();
                string sql = "SELECT * FROM country";

                using (MySqlCommand cmdSel = new MySqlCommand(sql, connection))
                {
                    DataTable dt = new DataTable();
                    MySqlDataAdapter da = new MySqlDataAdapter(cmdSel);


                    da.Fill(dt);
                    datagrid1.ItemsSource = dt.DefaultView;
                }
                connection.Close();
            }
        }
        public void writexl()
        {
            server = "localhost";
            database = "world";
            user = "root";
            password = "v?557ZX";
            port = "3306";

            connectionString = $"server={server};port={port};user id={user}; password={password}; database={database};";

            using (MySqlConnection connection = new MySqlConnection(connectionString))
            {
                connection.Open();
                string sql = "SELECT Name,Continent, COALESCE(LifeExpectancy, 0.0) AS LifeExpectancy FROM world.country";


                using (MySqlCommand cmdSel = new MySqlCommand(sql, connection))
                {
                    DataTable dt = new DataTable();
                    MySqlDataAdapter da = new MySqlDataAdapter(cmdSel);

                    da.Fill(dt);
                    DataSet myDataSet = new DataSet();

                    myDataSet.Tables.Add(dt);

                    DataSetHelper.CreateWorkbook("C:\\Users\\mls\\Desktop\\myExcel.xls", myDataSet);
                    MessageBox.Show("creating excel");
                }
                connection.Close();
            }
        }
    }
}