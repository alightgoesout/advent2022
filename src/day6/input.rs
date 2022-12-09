pub const INPUT: &str = "tnmmpfmfzmmnsmsjmjjbvvhnhzzfmmgpmgpgbgnnwffjhffzqqmzzbnbssrqqrnnhsnngsszsqzszhzfhzfzwzfzrrmhmghgwhhjjqwqttwhttjllrtrtzzcfzfgzznfznfzfnnbddvmvzmmfsmfsmfffhlfldlqqrnrznnhmmgqqzhhmjhmhppqbpbbngnlldvvdqvvrtrdrtrnttnppfllrbbrprpnpdplpmllhwwddqpdprddzzfccqpcqpcpcbbdhdjdjwjcwcctdcttzgzmmscmsmdmttwhwzhhnjhnhlhvhlvlglpgpmmjmgmrgrddmwddjfftfwflfslffqtfqttpftppflfmmhvhvcvbvhbhggpbgbppvdpvpvfppbwwsnnhphllbdbnbvbmvvzffvsffdldmlmtmccnlnbnjbnjnhhbfhhgzzlwlfflzffdccggdcgcjjhffjfgfgcczjccvwcvvqgvvqvllqzqmqllhjjqnqggttsdddjgdjgjzzrgrfrbrssrgrgdgrgbbssmdsdfddsndnsdnsdnnmqqsspqqmrqqpmmsjmmszzqvqrvrzznnjdndtntfnttgtctqtwwnwswrrthrttsdttlhlvvdzzgqgttnppjpljplpgpvgvqqvppzmmqggtjgtgstslltjltjjgcjcmjmsshvvtppgmmlslqqshqshsllbggfpgffdsdgssncchctcwwtllgqlqblqlqvvmsvmmwnnzppqllsttgmttftvfvjjrzzswzzjvzjzljjchcshcscbbrdbrbcrrnvvtctntvtvbvjvqjqggsrspsprrbgghdghhmwwldldzdttrvrnrfftqtftrrdsszlzvvbtbffftzzrzqrrhjhghhwbhhsjsfsttdjdjnjhjmjpmplplrrdjdcdjdbjblllbqlqdlqlpqptppdhhqmqfqhqhchwwqjqfjqfqhqshsmswsbbvssdspdpsdssstntltrrgnnmttgmmsjjrlrnlrnrnwrwfwlfltlzllcjcmjcjpjhphcpcwppmvmjjzbzvbbfnfcflfddntddbmmmhnnsrnrrvdvnvcvwvcwvwrrqwqccqmmswmmjrjmmwjmjfjhwrtbjzdvlgrjmvzfmhcqsncvlhzzncjlbvcwrdwjmqjcnptqslvfzpsvltgzsvjdsjrppdrmqrbqwhddfhnftfblspsrhtdtjwdnhbcbtlwlvccsfscvczzrrqmwbwbdmwgzqntvflppqvppwrhnvtlsbzqglhsfdgssqzdtjdpwrrhbnbtwhhnmnlwfwlqffjjrndbpwwsvdrhddbjnnqzmtpvvtwbcpndjzlhcfrrdvmljswjzvmfqcdsgqwclqshwrmblszdvsnrpdgnllmlchzdjlrrpndmmgddjqgjqrhwfbwddqdfbvptrmzhtsqfsfswpnvmtswqprjhbzvntgrlzthhnqbtpplqpvcfnpgdtbhqbhflltbbtmmhcwztslmpznttmssclhmnbsbrwlblrbsdfmnpqbwwmsncvzmpqwhzjgcgdrzvglgdtswmstdhrprdjfmqtjlmplbjtzcgnrwpdvpfjjfwjfnnpmdtwtqsgfndngsbmcwjtglqwtfrclbczfcmjtgcwszhzrbcphrhwmhcwghjznzthnwpljjltdlvqtffsrbmwcsvrdmqqggbznnlzbbqtgspqvnjpbdhtzmgttrcwwszwpgdrcnfqtgrgqdrctlzwtdwqppbhnwgldnqltznnfpbfqtgmmwpcqnndbgmrrtgtvnmlfcwsldchjnnqfrhpzwtclrzftsqllgvpqbgmfjdhqjttwcvbpvfqsvhbhhtwnqnbgndbtzhcvgglbhghbzrbrmdllmgfgttqmhtdnwrpwllhnghrjctrbzrcpnjnctvmrlpjhftnfbczrjrnnbqplplcrbngbhvmmvcffmgvbhjzbhcmtwmwgmjmwjvvlqfldswpntjnsjvmdlbzqqlgbwspwvmnwtwjbczmwplrhmjgsppnmtwmvsfwnsgddgwqcvpftcpzrhpldnwmcjgtjmljjbcmjcqdbwczndnjnjgrmtjrqnnjndzqdqpcgdqptdbrqftnwrgqmrzrvsfmmmbpltlncvtgrjfjmvtgwqphczwjhdrdwtfvgztbhrndvpcbgfjfvmrrljwrvcrtdmtjndfnwgcnfrzgsnjpztbwwsbvqfnpjctgrhsflhnzbbsfqbnmtnvrmjzsbjfndvttpvpfjhqntflgbfnzcclcwmhbsgqfjdcgsvrhtstspfzgvgglgddqmclsmzgzgtncdsfmwdvtcsgwvbzjvclwppqdjgfcrcbzcwbdhrnssjbmnmfmwthdrnmlfhqlddwqrdhsdvdcsmcgjsgcmpnhlbnqftpdjswtmpbznlcrhtswgnmwjcdfmljdngzfsmlzjjnzmfzshmztdbdmcqwmlvcrzgpmbjqcghclwvdbrhgvwqchnndftnrtptmctdlhmfjvpzrpccddfpcdwmzqfhnsqzrvwblzfhcjdcjfctczwqrcbjnrpdcbbnsgnlvqqmnsfgsqschjlbzhhsrbvdbfrhvsgrlzwncgwpdbvmblgzbwbcbgqfwmdmgcrbbjfcvmqgztqpptdhwmvmsdqwplpgcjzgqzdrftzhqbltvhrmlrfffcgfpqzwrrbbtlsjgmtbjvtnmhwdpjptjwfwgjgvbfqwmflrrqzlzdcmtlnptdrpcpdnswcfscnndnrfbgwvvncdjgsdpbwptdtvrqlmrhmvvcwblhhzbjdpsbszhrftfbcgwhwrgglnjzqdhcqnvlhgqjhnddvrslhntssptsbhmqwwqqnbvfmcbgpvgjbrttnvlljdbtfplgmbwtcbcdtqdpqqdvhbmpmtszwpzblcfrtznhhtcljtdlhjdbnlhvwgjsmgvrslrfwnmzwlstpgltvrgnpdqztvfnvdhdtwwqdfsmtpbpdclsbnwcgjzchjcsjmvhbjshmjjlpgdzcgbmmchwmcsddsvhsnpqtcpnhqnbvwgwqhtjbqncgwwftnrzsbsjtvqmjzqvvncmncwflcfpcjqgdtbsmjzzsdjfvhnqbgjhmfgjghwscthbfmbndltbqzwpqtmrswvprpmgwqnqpfnmffrpdlpfqmhrthppzvzwbrtjvwvjndsqdlqtbpqwfcttggnjmcqqnmjwfhfjgcvlnmtlgbdvmctzlwbfgnflwtsflgnfbnfbhhdgjctzvvmrhdsmvmmtnqwtszmqcpsbrqrgjfrzctcbzmtdlhwjtfdqbtthdnqcrpwrhcrvjstbhpltvgmvpmvfjstgzjsgzprzcqzqztvvdcnrrqwrhddcrhhncdrlwzwqlnbbzcfmqtnwgfdscmrbwnbldlfrqchzdnlnmwncgrzdclnvcvplgwjsbzmbnnsdrsfhrlssvncnwmcrjdjbjpdtrrvlnbjvspfqbwdpcnnpjzfnmbhcdhlmdgbpvbzmfltzstnznfctcdzhbfsvnfbsjqzmwfllhtrsfghlrpjgrgzgchlwrdmqzbrncsvnwhfqmwjbnvjctzphcsftqsbmwntgvjqhhvwndvmfmjhhhmfdvrlhpvzmmhrbhbddqbdmgqqsvddsswmzqcjmvhztfqpchzpwhdshzjlmbmnsgzqhbnmrshwvtmgmgndtddpfwsjrrjdhncdhtlczdvlbvqplttnzrblthlcffdtfsdtpwzdgbldvnsttvpzmbgnqddrszftcpwrgmfzhjjvghpntmzcttcsnrjnfpqzqqqljhzlrpgwngllqjwnwfcsphqplgbzmfqfgbfsqpsrntszqbcqnhctsnbfshmlbwfflrwwsjwqwfqlgnftdwmctmclwjhjhbsspqldlshbmpbgrftpnbpsqldhrrbdqwfwvfhclrlfdjfmzgmptdjdcsplcspznfjrfhtsjndwpslrdgnllllwqjgznrhswfssdlvdpmwwgmstqbhfmdhtzvzzvhwzbrrvvsl";