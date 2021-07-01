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
    var projection = encodeURIComponent($("#projection_input").val());
    var finalUrl = url+"?collection="+collection+"&projection="+projection+"&"+formData;
    window.location.href = finalUrl;
}) 


$(document).ready(function (){
    $(".link").on("click", function (){
        if ($(this).height() == 200) {
            var el = $(this),
                curHeight = el.height(),
                autoHeight = el.css('height', 'auto').height();

            el.height(curHeight).animate(
                {height: autoHeight});
            }
        else if ($(this).height() != 200) {
            $(this).animate({height: "200px"});
            }
        });
    });

$("#menu_icon").click(function(){
  $("#options").toggle();
});
