how to use:  

  
rdbcli file.rdb -d data_name | to get data digit  
rdbcli file.rdb -m data_name | to get data meta  
rdbcli file.rdb -a data_name | to get data array  
  
To add data:  
rdbcli file.rdb -nd /name_M/=test=data=for=the=data  
  
To change data:   
rdbcli file.rdb -cd name data=changed  
  
To remove data:  
rdbcli file.rdb -rd name
  