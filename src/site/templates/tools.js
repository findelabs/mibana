$('#collections').change(function(){
      var url = "/query?";
      
      if($("#collection").val()!='Select')
        url+='collection='+encodeURIComponent($("#collections").val())+'&';
      
      url = url.replace(/\&$/,'');
      window.location.href=url;
});

$('#search_field').on('submit',function(e){
    e.preventDefault();
    var formData=$(this).serialize();
    var url = [location.protocol, '//', location.host, location.pathname].join('');
    var collection = urlParams.get('collection')
    var finalUrl = url+"?collection="+collection+"&"+formData;
    window.location.href = finalUrl;
}) 
