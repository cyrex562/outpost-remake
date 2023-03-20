
#include "init_globals.h"

#include "address_tables/function_tables.h"
#include "globals.h"
#include "op_int.h"


void init_globals_1020_96d4(Globals *globals)

{
    unsigned short pu_var_1;
    i16  i_var_2;
    u16 pu_var_3;

    globals->dat_1050_4514       = 0x0;
    globals->dat_1050_451a       = 0x0;
    globals->ptr_1050_4520       = data_1050_4430;//0x4430;
    globals->seg_addr_1050_4522  = SEG_1050;
    globals->ptr_1050_4526       = data_1050_4430;//0x4430;
    globals->seg_addr_1050_4528  = SEG_1050;
    globals->ptr_1050_4524       = data_1050_4434;
    globals->ptr_1050_452a       = data_1050_4434;
    globals->ptr_1050_452c       = data_1050_4430;//0x4430
    globals->seg_addr_1050_452e  = SEG_1050;
    globals->ptr_1050_4530       = data_1050_4434;
    globals->ptr_1050_4532       = data_1050_4430;//0x4430;
    globals->seg_addr_1050_4534  = SEG_1050;
    globals->ptr_1050_4536       = data_1050_4434;
    globals->_PTR_LOOP_1050_4538 = 0x0;
    globals->_PTR_LOOP_1050_453e = 0x0;
    globals->ptr_1050_4544       = data_1050_4436;//0x4436;
    globals->seg_addr_1050_4546  = SEG_1050;
    globals->ptr_1050_454a       = data_1050_4436;//0x4436;
    globals->seg_addr_1050_454c  = SEG_1050;
    globals->ptr_1050_4548       = data_1050_443a;
    globals->ptr_1050_454e       = data_1050_443a;
    globals->ptr_1050_4550       = data_1050_4436;//0x4436;
    globals->seg_addr_1050_4552  = SEG_1050;
    globals->ptr_1050_4554       = data_1050_443a;
    globals->ptr_1050_4512       = data_1050_4462;
    globals->ptr_1050_455a       = data_1050_4462;
    globals->ptr_1050_4556       = data_1050_4454;//0x4454;
    globals->seg_addr_1050_4558  = SEG_1050;
    globals->ptr_1050_455c       = data_1050_4454;//0x4454;
    globals->ptr_1050_455e       = SEG_1050;
    globals->ptr_1050_4560       = data_1050_4462;
    globals->ptr_1050_4562       = data_1050_4454;//0x4454;
    globals->seg_addr_1050_4564  = SEG_1050;
    globals->ptr_1050_4566       = data_1050_4462;
    globals->dat_1050_456a       = 0x0;
    globals->dat_1050_4568       = 0x0;
    globals->PTR_LOOP_1050_456e  = data_1050_443c;//0x443c;
    globals->PTR_LOOP_1050_4570  = SEG_1050;
    globals->PTR_LOOP_1050_4574  = data_1050_443c;//0x443c;
    globals->PTR_LOOP_1050_4576  = SEG_1050;
    globals->PTR_LOOP_1050_4572  = data_1050_4446;
    globals->PTR_LOOP_1050_4578  = data_1050_4446;
    globals->PTR_LOOP_1050_457a  = data_1050_443c;//0x443c;
    globals->PTR_LOOP_1050_457c  = SEG_1050;
    globals->PTR_LOOP_1050_457e  = data_1050_4446;
    globals->PTR_LOOP_1050_4580  = data_1050_443c;//0x443c;
    globals->PTR_LOOP_1050_4582  = SEG_1050;
    globals->PTR_LOOP_1050_4584  = data_1050_4446;
    globals->PTR_LOOP_1050_4586  = data_1050_443c;//0x443c;
    globals->PTR_LOOP_1050_4588  = SEG_1050;
    globals->PTR_LOOP_1050_458a  = data_1050_4446;
    globals->PTR_LOOP_1050_458c  = data_1050_443c;//0x443c;
    globals->PTR_LOOP_1050_458e  = SEG_1050;
    globals->PTR_LOOP_1050_4590  = data_1050_4446;
    globals->PTR_LOOP_1050_4592  = data_1050_4454;//0x4454;
    globals->PTR_LOOP_1050_4594  = SEG_1050;
    globals->PTR_LOOP_1050_4596  = data_1050_4462;
    globals->PTR_LOOP_1050_4598  = data_1050_4454;//0x4454;
    globals->PTR_LOOP_1050_459a  = SEG_1050;
    globals->PTR_LOOP_1050_459c  = data_1050_4462;
    globals->PTR_LOOP_1050_459e  = data_1050_4436;//0x4436;
    globals->PTR_LOOP_1050_45a0  = SEG_1050;
    globals->PTR_LOOP_1050_45a2  = data_1050_443a;
    globals->PTR_LOOP_1050_45a4  = data_1050_4436;//0x4436;
    globals->PTR_LOOP_1050_45a6  = SEG_1050;
    globals->PTR_LOOP_1050_45a8  = data_1050_443a;
    globals->_PTR_LOOP_1050_45aa = 0x0;
    globals->_PTR_LOOP_1050_45b0 = 0x0;
    globals->_PTR_LOOP_1050_45b6 = 0x0;
    globals->PTR_LOOP_1050_45bc  = data_1050_443c;//0x443c;
    globals->PTR_LOOP_1050_45be  = SEG_1050;
    globals->PTR_LOOP_1050_45c0  = data_1050_4446;
    globals->PTR_LOOP_1050_45c2  = data_1050_443c;//0x443c;
    globals->PTR_LOOP_1050_45c4  = SEG_1050;
    globals->PTR_LOOP_1050_45c6  = data_1050_4446;
    globals->_PTR_LOOP_1050_45c8 = 0x0;
    globals->_PTR_LOOP_1050_45ce = 0x0;
    globals->_PTR_LOOP_1050_45d4 = 0x0;
    globals->_PTR_LOOP_1050_45da = 0x0;
    globals->PTR_LOOP_1050_45e0  = data_1050_443c;//0x443c;
    globals->PTR_LOOP_1050_45e2  = SEG_1050;
    globals->PTR_LOOP_1050_45e4  = data_1050_4446;
    globals->PTR_LOOP_1050_45e6  = data_1050_443c;//0x443c;
    globals->PTR_LOOP_1050_45e8  = SEG_1050;
    globals->PTR_LOOP_1050_45ea  = data_1050_4446;
    globals->_PTR_LOOP_1050_45ec = 0x0;
    globals->_PTR_LOOP_1050_45f2 = 0x0;
    globals->_PTR_LOOP_1050_45f8 = 0x0;
    globals->PTR_LOOP_1050_45fe  = data_1050_443c;//0x443c;
    globals->PTR_LOOP_1050_4600  = SEG_1050;
    globals->PTR_LOOP_1050_4602  = data_1050_4446;
    globals->PTR_LOOP_1050_4604  = data_1050_443c;//0x443c;
    globals->PTR_LOOP_1050_4606  = SEG_1050;
    globals->PTR_LOOP_1050_4608  = data_1050_4446;
    globals->_PTR_LOOP_1050_460a = 0x0;
    globals->_PTR_LOOP_1050_4610 = 0x0;
    globals->PTR_LOOP_1050_451e  = 0xffff;
    globals->PTR_LOOP_1050_45ae  = 0xffff;
    globals->PTR_LOOP_1050_45b4  = 0xffff;
    globals->PTR_LOOP_1050_45ba  = 0xffff;
    globals->PTR_LOOP_1050_45cc  = 0xffff;
    globals->PTR_LOOP_1050_45d2  = 0xffff;
    globals->PTR_LOOP_1050_45f6  = 0xffff;
    globals->PTR_LOOP_1050_45fc  = 0xffff;
    globals->PTR_LOOP_1050_460e  = 0xffff;
    globals->PTR_LOOP_1050_4614  = 0xffff;
    globals->_PTR_LOOP_1050_4616 = 0x0;
    globals->_PTR_LOOP_1050_461c = 0x0;
    globals->_PTR_LOOP_1050_4622 = 0x0;
    globals->_PTR_LOOP_1050_4628 = 0x0;
    globals->_PTR_LOOP_1050_462e = 0x0;
    globals->_PTR_LOOP_1050_4634 = 0x0;
    globals->PTR_LOOP_1050_4518  = 0x0;
    globals->PTR_LOOP_1050_453c  = 0x0;
    globals->PTR_LOOP_1050_4542  = 0x0;
    globals->PTR_LOOP_1050_456c  = 0x0;
    globals->PTR_LOOP_1050_45d8  = 0x0;
    globals->PTR_LOOP_1050_45de  = 0x0;
    globals->PTR_LOOP_1050_45f0  = 0x0;
    globals->PTR_LOOP_1050_461a  = 0x0;
    globals->PTR_LOOP_1050_4620  = 0x0;
    globals->PTR_LOOP_1050_4626  = 0x0;
    globals->PTR_LOOP_1050_462c  = 0x0;
    globals->PTR_LOOP_1050_4632  = 0x0;
    globals->PTR_LOOP_1050_4638  = 0x0;
    globals->_PTR_LOOP_1050_463a = 0x0;
    globals->_PTR_LOOP_1050_4640 = 0x0;
    globals->_PTR_LOOP_1050_4646 = 0x0;
    globals->_PTR_LOOP_1050_464c = 0x0;
    globals->_PTR_LOOP_1050_4652 = 0x0;
    globals->_PTR_LOOP_1050_4658 = 0x0;
    globals->PTR_LOOP_1050_465e  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_4660  = SEG_1050;
    globals->PTR_LOOP_1050_4664  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_4666  = SEG_1050;
    globals->PTR_LOOP_1050_4662  = data_1050_4452;
    globals->PTR_LOOP_1050_4668  = data_1050_4452;
    globals->PTR_LOOP_1050_466a  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_466c  = SEG_1050;
    globals->PTR_LOOP_1050_466e  = data_1050_4452;
    globals->PTR_LOOP_1050_4670  = data_1050_4454;//0x4454;
    globals->PTR_LOOP_1050_4672  = SEG_1050;
    globals->PTR_LOOP_1050_4676  = data_1050_4454;//0x4454;
    globals->PTR_LOOP_1050_4678  = SEG_1050;
    globals->PTR_LOOP_1050_4674  = data_1050_4462;
    globals->PTR_LOOP_1050_467a  = data_1050_4462;
    globals->PTR_LOOP_1050_467c  = data_1050_4454;//0x4454;
    globals->PTR_LOOP_1050_467e  = SEG_1050;
    globals->PTR_LOOP_1050_4680  = data_1050_4462;
    globals->PTR_LOOP_1050_4682  = data_1050_4454;//0x4454;
    globals->PTR_LOOP_1050_4684  = SEG_1050;
    globals->PTR_LOOP_1050_4686  = data_1050_4462;
    globals->PTR_LOOP_1050_4688  = data_1050_4454;//0x4454;
    globals->PTR_LOOP_1050_468a  = SEG_1050;
    globals->PTR_LOOP_1050_468c  = data_1050_4462;
    globals->PTR_LOOP_1050_468e  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_4690  = SEG_1050;
    globals->PTR_LOOP_1050_4692  = data_1050_4452;
    globals->PTR_LOOP_1050_4694  = data_1050_4448;//0x4448
    globals->PTR_LOOP_1050_4696  = SEG_1050;
    globals->PTR_LOOP_1050_4698  = data_1050_4452;
    globals->PTR_LOOP_1050_469a  = data_1050_4448;//0x4448
    globals->PTR_LOOP_1050_469c  = SEG_1050;
    globals->PTR_LOOP_1050_469e  = data_1050_4452;
    globals->PTR_LOOP_1050_46a0  = data_1050_4448;//0x4448
    globals->PTR_LOOP_1050_46a2  = SEG_1050;
    globals->PTR_LOOP_1050_46a4  = data_1050_4452;
    globals->PTR_LOOP_1050_46a6  = data_1050_4454;//0x4454;
    globals->PTR_LOOP_1050_46a8  = SEG_1050;
    globals->PTR_LOOP_1050_46aa  = data_1050_4462;
    globals->PTR_LOOP_1050_46ac  = data_1050_4454;//0x4454;
    globals->PTR_LOOP_1050_46ae  = SEG_1050;
    globals->PTR_LOOP_1050_46b0  = data_1050_4462;
    globals->PTR_LOOP_1050_46b2  = data_1050_4454;//0x4454;
    globals->PTR_LOOP_1050_46b4  = SEG_1050;
    globals->PTR_LOOP_1050_46b6  = data_1050_4462;
    globals->PTR_LOOP_1050_46b8  = data_1050_4454;//0x4454;
    globals->PTR_LOOP_1050_46ba  = SEG_1050;
    globals->PTR_LOOP_1050_46bc  = data_1050_4462;
    globals->PTR_LOOP_1050_46be  = data_1050_4454;//0x4454;
    globals->PTR_LOOP_1050_46c0  = SEG_1050;
    globals->PTR_LOOP_1050_46c2  = data_1050_4462;
    globals->PTR_LOOP_1050_46c6  = 0x0;
    globals->PTR_LOOP_1050_46c4  = 0x0;
    globals->PTR_LOOP_1050_46cc  = 0x0;
    globals->PTR_LOOP_1050_46ca  = 0x0;
    globals->PTR_LOOP_1050_46d2  = 0x0;
    globals->PTR_LOOP_1050_46d0  = 0x0;
    globals->PTR_LOOP_1050_46d8  = 0x0;
    globals->PTR_LOOP_1050_46d6  = 0x0;
    globals->PTR_LOOP_1050_46de  = 0x0;
    globals->PTR_LOOP_1050_46dc  = 0x0;
    globals->PTR_LOOP_1050_46e2  = data_1050_4454;//0x4454;
    globals->PTR_LOOP_1050_46e4  = SEG_1050;
    globals->PTR_LOOP_1050_46e6  = data_1050_4462;
    globals->PTR_LOOP_1050_46e8  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_46ea  = SEG_1050;
    globals->PTR_LOOP_1050_46ec  = data_1050_4452;
    globals->PTR_LOOP_1050_46ee  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_46f0  = SEG_1050;
    globals->PTR_LOOP_1050_46f2  = data_1050_4452;
    globals->_PTR_LOOP_1050_46f4 = 0x0;
    globals->_PTR_LOOP_1050_46fa = 0x0;
    globals->PTR_LOOP_1050_46f8  = 0xffff;
    globals->PTR_LOOP_1050_46fe  = 0xffff;
    globals->_PTR_LOOP_1050_4700 = 0x0;
    globals->_PTR_LOOP_1050_4706 = 0x0;
    globals->PTR_LOOP_1050_470c  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_470e  = SEG_1050;
    globals->PTR_LOOP_1050_4710  = data_1050_4452;
    globals->PTR_LOOP_1050_4712  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_4714  = SEG_1050;
    globals->PTR_LOOP_1050_4716  = data_1050_4452;
    globals->_PTR_LOOP_1050_4718 = 0x0;
    globals->_PTR_LOOP_1050_471e = 0x0;
    globals->_PTR_LOOP_1050_4724 = 0x0;
    globals->_PTR_LOOP_1050_472a = 0x0;
    globals->_PTR_LOOP_1050_4730 = 0x0;
    globals->_PTR_LOOP_1050_4736 = 0x0;
    globals->_PTR_LOOP_1050_473c = 0x0;
    globals->_PTR_LOOP_1050_4742 = 0x0;
    globals->_PTR_LOOP_1050_4748 = 0x0;
    globals->_PTR_LOOP_1050_474e = 0x0;
    globals->_PTR_LOOP_1050_4754 = 0x0;
    globals->_PTR_LOOP_1050_475a = 0x0;
    globals->_PTR_LOOP_1050_4760 = 0x0;
    globals->PTR_LOOP_1050_463e  = 0x0;
    globals->PTR_LOOP_1050_4644  = 0x0;
    globals->PTR_LOOP_1050_464a  = 0x0;
    globals->PTR_LOOP_1050_4650  = 0x0;
    globals->PTR_LOOP_1050_4656  = 0x0;
    globals->PTR_LOOP_1050_465c  = 0x0;
    globals->PTR_LOOP_1050_46c8  = 0x0;
    globals->PTR_LOOP_1050_46ce  = 0x0;
    globals->PTR_LOOP_1050_46d4  = 0x0;
    globals->PTR_LOOP_1050_46da  = 0x0;
    globals->PTR_LOOP_1050_46e0  = 0x0;
    globals->PTR_LOOP_1050_4704  = 0x0;
    globals->PTR_LOOP_1050_470a  = 0x0;
    globals->PTR_LOOP_1050_471c  = 0x0;
    globals->PTR_LOOP_1050_4722  = 0x0;
    globals->PTR_LOOP_1050_4728  = 0x0;
    globals->PTR_LOOP_1050_472e  = 0x0;
    globals->PTR_LOOP_1050_4734  = 0x0;
    globals->PTR_LOOP_1050_473a  = 0x0;
    globals->PTR_LOOP_1050_4740  = 0x0;
    globals->PTR_LOOP_1050_4746  = 0x0;
    globals->PTR_LOOP_1050_474c  = 0x0;
    globals->PTR_LOOP_1050_4752  = 0x0;
    globals->PTR_LOOP_1050_4758  = 0x0;
    globals->PTR_LOOP_1050_475e  = 0x0;
    globals->PTR_LOOP_1050_4764  = 0x0;
    globals->_PTR_LOOP_1050_4766 = 0x0;
    globals->_PTR_LOOP_1050_476c = 0x0;
    globals->_PTR_LOOP_1050_4772 = 0x0;
    globals->_PTR_LOOP_1050_4778 = 0x0;
    globals->_PTR_LOOP_1050_477e = 0x0;
    globals->_PTR_LOOP_1050_4784 = 0x0;
    globals->_PTR_LOOP_1050_478a = 0x0;
    globals->_PTR_LOOP_1050_4790 = 0x0;
    globals->_PTR_LOOP_1050_4796 = 0x0;
    globals->_PTR_LOOP_1050_479c = 0x0;
    globals->_PTR_LOOP_1050_47a2 = 0x0;
    globals->_PTR_LOOP_1050_47a8 = 0x0;
    globals->_PTR_LOOP_1050_47ae = 0x0;
    globals->_PTR_LOOP_1050_47b4 = 0x0;
    globals->PTR_LOOP_1050_476a  = 0x0;
    globals->PTR_LOOP_1050_4770  = 0x0;
    globals->PTR_LOOP_1050_4776  = 0x0;
    globals->PTR_LOOP_1050_477c  = 0x0;
    globals->PTR_LOOP_1050_4782  = 0x0;
    globals->PTR_LOOP_1050_4788  = 0x0;
    globals->PTR_LOOP_1050_478e  = 0x0;
    globals->PTR_LOOP_1050_4794  = 0x0;
    globals->PTR_LOOP_1050_479a  = 0x0;
    globals->PTR_LOOP_1050_47a0  = 0x0;
    globals->PTR_LOOP_1050_47a6  = 0x0;
    globals->PTR_LOOP_1050_47ac  = 0x0;
    globals->PTR_LOOP_1050_47b2  = 0x0;
    globals->PTR_LOOP_1050_47b8  = 0x0;
//    pu_var_3                     = 0x47ba;
//    for(i_var_2 = 0x1b; i_var_2 != 0x0; i_var_2 = i_var_2 + -0x1)
//    {
//        pu_var_1  = pu_var_3;
//        pu_var_3  = pu_var_3 + 0x1;
//        *pu_var_1 = 0x0;
//    }
    globals->_PTR_LOOP_1050_4850 = 0x0;
    globals->_PTR_LOOP_1050_4856 = 0x0;
    globals->PTR_LOOP_1050_484e  = data_1050_4468;
    globals->PTR_LOOP_1050_4860  = data_1050_4468;
    globals->PTR_LOOP_1050_485c  = data_1050_4464;//0x4464;
    globals->PTR_LOOP_1050_485e  = SEG_1050;
    globals->PTR_LOOP_1050_4862  = data_1050_4464;//0x4464;
    globals->PTR_LOOP_1050_4864  = SEG_1050;
    globals->PTR_LOOP_1050_4866  = data_1050_4468;
    globals->PTR_LOOP_1050_4868  = data_1050_4464;//0x4464;
    globals->PTR_LOOP_1050_486a  = SEG_1050;
    globals->PTR_LOOP_1050_486c  = data_1050_4468;
    globals->PTR_LOOP_1050_486e  = data_1050_4464;//0x4464;
    globals->PTR_LOOP_1050_4870  = SEG_1050;
    globals->PTR_LOOP_1050_4872  = data_1050_4468;
    globals->_PTR_LOOP_1050_4874 = 0x0;
    globals->_PTR_LOOP_1050_487a = 0x0;
    globals->PTR_LOOP_1050_4880  = data_1050_4436;//0x4436;
    globals->PTR_LOOP_1050_4882  = SEG_1050;
    globals->PTR_LOOP_1050_4886  = data_1050_4436;//0x4436;
    globals->PTR_LOOP_1050_4888  = SEG_1050;
    globals->PTR_LOOP_1050_4884  = data_1050_443a;
    globals->PTR_LOOP_1050_488a  = data_1050_443a;
    globals->PTR_LOOP_1050_488c  = data_1050_4436;//0x4436;
    globals->PTR_LOOP_1050_488e  = SEG_1050;
    globals->PTR_LOOP_1050_4890  = data_1050_443a;
    globals->PTR_LOOP_1050_4892  = data_1050_4482;//0x4482;
    globals->PTR_LOOP_1050_4894  = SEG_1050;
    globals->PTR_LOOP_1050_4898  = data_1050_4482;//0x4482;
    globals->PTR_LOOP_1050_489a  = SEG_1050;
    globals->PTR_LOOP_1050_4896  = data_1050_4486;
    globals->PTR_LOOP_1050_489c  = data_1050_4486;
    globals->PTR_LOOP_1050_489e  = data_1050_4482;//0x4482;
    globals->PTR_LOOP_1050_48a0  = SEG_1050;
    globals->PTR_LOOP_1050_48a2  = data_1050_4486;
    globals->PTR_LOOP_1050_48a6  = 0x0;
    globals->PTR_LOOP_1050_48a4  = 0x0;
    globals->PTR_LOOP_1050_48aa  = data_1050_4488;//0x4488;
    globals->PTR_LOOP_1050_48ac  = SEG_1050;
    globals->PTR_LOOP_1050_48b0  = data_1050_4488;//0x4488;
    globals->PTR_LOOP_1050_48b2  = SEG_1050;
    globals->PTR_LOOP_1050_48ae  = data_1050_448c;
    globals->PTR_LOOP_1050_48b4  = data_1050_448c;
    globals->PTR_LOOP_1050_48b6  = data_1050_4488;//0x4488;
    globals->PTR_LOOP_1050_48b8  = SEG_1050;
    globals->PTR_LOOP_1050_48ba  = data_1050_448c;
    globals->PTR_LOOP_1050_48bc  = data_1050_446a;//;0x446a;
    globals->PTR_LOOP_1050_48be  = SEG_1050;
    globals->PTR_LOOP_1050_48c2  = data_1050_446a;
    globals->PTR_LOOP_1050_48c4  = SEG_1050;
    globals->PTR_LOOP_1050_48c0  = globals->PTR_LOOP_1050_446e;
    globals->PTR_LOOP_1050_48c6  = globals->PTR_LOOP_1050_446e;
    globals->PTR_LOOP_1050_48c8  = data_1050_446a;
    globals->PTR_LOOP_1050_48ca  = SEG_1050;
    globals->PTR_LOOP_1050_48cc  = globals->PTR_LOOP_1050_446e;
    globals->PTR_LOOP_1050_48ce  = data_1050_447a;//0x447a;
    globals->PTR_LOOP_1050_48d0  = SEG_1050;
    globals->PTR_LOOP_1050_48d4  = data_1050_447a;//0x447a;
    globals->PTR_LOOP_1050_48d6  = SEG_1050;
    globals->PTR_LOOP_1050_48d2  = data_1050_4480;
    globals->PTR_LOOP_1050_48d8  = data_1050_4480;
    globals->PTR_LOOP_1050_48da  = data_1050_4436;//0x4436;
    globals->PTR_LOOP_1050_48dc  = SEG_1050;
    globals->PTR_LOOP_1050_48de  = data_1050_443a;
    globals->PTR_LOOP_1050_48e0  = data_1050_4436;//0x4436;
    globals->PTR_LOOP_1050_48e2  = SEG_1050;
    globals->PTR_LOOP_1050_48e4  = data_1050_443a;
    globals->PTR_LOOP_1050_48e6  = data_1050_447a;//0x447a;
    globals->PTR_LOOP_1050_48e8  = SEG_1050;
    globals->PTR_LOOP_1050_48ea  = data_1050_4480;
    globals->_PTR_LOOP_1050_48ec = 0x0;
    globals->_PTR_LOOP_1050_48f2 = 0x0;
    globals->PTR_LOOP_1050_48f8  = data_1050_447a;//0x447a;
    globals->PTR_LOOP_1050_48fa  = SEG_1050;
    globals->PTR_LOOP_1050_48fc  = data_1050_4480;
    globals->PTR_LOOP_1050_48fe  = data_1050_447a;
    globals->PTR_LOOP_1050_4900  = SEG_1050;
    globals->PTR_LOOP_1050_4902  = data_1050_4480;
    globals->_PTR_LOOP_1050_4904 = 0x0;
    globals->_PTR_LOOP_1050_490a = 0x0;
    globals->PTR_LOOP_1050_485a  = 0xffff;
    globals->PTR_LOOP_1050_48f0  = 0xffff;
    globals->PTR_LOOP_1050_48f6  = 0xffff;
    globals->PTR_LOOP_1050_4908  = 0xffff;
    globals->PTR_LOOP_1050_490e  = 0xffff;
    globals->_PTR_LOOP_1050_4910 = 0x0;
    globals->_PTR_LOOP_1050_4916 = 0x0;
    globals->PTR_LOOP_1050_4854  = 0x0;
    globals->PTR_LOOP_1050_4878  = 0x0;
    globals->PTR_LOOP_1050_487e  = 0x0;
    globals->PTR_LOOP_1050_48a8  = 0x0;
    globals->PTR_LOOP_1050_4914  = 0x0;
    globals->PTR_LOOP_1050_491a  = 0x0;
    globals->PTR_LOOP_1050_491c  = data_1050_4488;//0x4488;
    globals->PTR_LOOP_1050_491e  = SEG_1050;
    globals->PTR_LOOP_1050_4920  = data_1050_448c;
    globals->PTR_LOOP_1050_4922  = data_1050_4488;//0x4488;
    globals->PTR_LOOP_1050_4924  = SEG_1050;
    globals->PTR_LOOP_1050_4926  = data_1050_448c;
    globals->_PTR_LOOP_1050_4928 = 0x0;
    globals->_PTR_LOOP_1050_492e = 0x0;
    globals->_PTR_LOOP_1050_4934 = 0x0;
    globals->PTR_LOOP_1050_493a  = data_1050_446a;//0x446a;
    globals->PTR_LOOP_1050_493c  = SEG_1050;
    globals->PTR_LOOP_1050_4940  = data_1050_446a;//0x446a;
    globals->PTR_LOOP_1050_4942  = SEG_1050;
    globals->PTR_LOOP_1050_493e  = data_1050_446e;
    globals->PTR_LOOP_1050_4944  = data_1050_446e;
    globals->_PTR_LOOP_1050_4946 = 0x0;
    globals->_PTR_LOOP_1050_494c = 0x0;
    globals->_PTR_LOOP_1050_4952 = 0x0;
    globals->_PTR_LOOP_1050_4958 = 0x0;
    globals->_PTR_LOOP_1050_495e = 0x0;
    globals->_PTR_LOOP_1050_4964 = 0x0;
    globals->_PTR_LOOP_1050_496a = 0x0;
    globals->_PTR_LOOP_1050_4970 = 0x0;
    globals->_PTR_LOOP_1050_4976 = 0x0;
    globals->_PTR_LOOP_1050_497c = 0x0;
    globals->_PTR_LOOP_1050_4982 = 0x0;
    globals->_PTR_LOOP_1050_4988 = 0x0;
    globals->_PTR_LOOP_1050_498e = 0x0;
    globals->_PTR_LOOP_1050_4994 = 0x0;
    globals->PTR_LOOP_1050_499a  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_499c  = SEG_1050;
    globals->PTR_LOOP_1050_49a0  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_49a2  = SEG_1050;
    globals->PTR_LOOP_1050_499e  = data_1050_4452;
    globals->PTR_LOOP_1050_49a4  = data_1050_4452;
    globals->PTR_LOOP_1050_49a6  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_49a8  = SEG_1050;
    globals->PTR_LOOP_1050_49aa  = data_1050_4452;
    globals->PTR_LOOP_1050_49ac  = data_1050_4470;//0x4470;
    globals->PTR_LOOP_1050_49ae  = SEG_1050;
    globals->PTR_LOOP_1050_49b2  = data_1050_4470;//0x4470;
    globals->PTR_LOOP_1050_49b4  = SEG_1050;
    globals->PTR_LOOP_1050_49b0  = data_1050_4478;
    globals->PTR_LOOP_1050_49b6  = data_1050_4478;
    globals->PTR_LOOP_1050_49b8  = data_1050_4470;//0x4470;
    globals->PTR_LOOP_1050_49ba  = SEG_1050;
    globals->PTR_LOOP_1050_49bc  = data_1050_4478;
    globals->PTR_LOOP_1050_49be  = data_1050_4470;//0x4470;
    globals->PTR_LOOP_1050_49c0  = SEG_1050;
    globals->PTR_LOOP_1050_49c2  = data_1050_4478;
    globals->PTR_LOOP_1050_49c4  = data_1050_4470;//0x4470;
    globals->PTR_LOOP_1050_49c6  = SEG_1050;
    globals->PTR_LOOP_1050_49c8  = data_1050_4478;
    globals->PTR_LOOP_1050_49ca  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_49cc  = SEG_1050;
    globals->PTR_LOOP_1050_49ce  = data_1050_4452;
    globals->PTR_LOOP_1050_49d0  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_49d2  = SEG_1050;
    globals->PTR_LOOP_1050_49d4  = data_1050_4452;
    globals->PTR_LOOP_1050_49d6  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_49d8  = SEG_1050;
    globals->PTR_LOOP_1050_49da  = data_1050_4452;
    globals->PTR_LOOP_1050_49dc  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_49de  = SEG_1050;
    globals->PTR_LOOP_1050_49e0  = data_1050_4452;
    globals->PTR_LOOP_1050_49e2  = data_1050_4482;//0x4482;
    globals->PTR_LOOP_1050_49e4  = SEG_1050;
    globals->PTR_LOOP_1050_49e8  = data_1050_4482;//0x4482;
    globals->PTR_LOOP_1050_49ea  = SEG_1050;
    globals->PTR_LOOP_1050_49e6  = data_1050_4486;
    globals->PTR_LOOP_1050_49ec  = data_1050_4486;
    globals->PTR_LOOP_1050_49ee  = data_1050_4470;//0x4470;
    globals->PTR_LOOP_1050_49f0  = SEG_1050;
    globals->PTR_LOOP_1050_49f2  = data_1050_4478;
    globals->PTR_LOOP_1050_49f4  = data_1050_4470;//0x4470;
    globals->PTR_LOOP_1050_49f6  = SEG_1050;
    globals->PTR_LOOP_1050_49f8  = data_1050_4478;
    globals->PTR_LOOP_1050_49fa  = data_1050_4470;//0x4470;
    globals->PTR_LOOP_1050_49fc  = SEG_1050;
    globals->PTR_LOOP_1050_49fe  = data_1050_4478;
    globals->PTR_LOOP_1050_4a02  = 0x0;
    globals->PTR_LOOP_1050_4a00  = 0x0;
    globals->PTR_LOOP_1050_4a08  = 0x0;
    globals->PTR_LOOP_1050_4a06  = 0x0;
    globals->PTR_LOOP_1050_4a0e  = 0x0;
    globals->PTR_LOOP_1050_4a0c  = 0x0;
    globals->PTR_LOOP_1050_4a14  = 0x0;
    globals->PTR_LOOP_1050_4a12  = 0x0;
    globals->PTR_LOOP_1050_4a1a  = 0x0;
    globals->PTR_LOOP_1050_4a18  = 0x0;
    globals->PTR_LOOP_1050_4a1e  = data_1050_4470;//0x4470;
    globals->PTR_LOOP_1050_4a20  = SEG_1050;
    globals->PTR_LOOP_1050_4a22  = data_1050_4478;
    globals->PTR_LOOP_1050_4a24  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_4a26  = SEG_1050;
    globals->PTR_LOOP_1050_4a28  = data_1050_4452;
    globals->PTR_LOOP_1050_4a2a  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_4a2c  = SEG_1050;
    globals->PTR_LOOP_1050_4a2e  = data_1050_4452;
    globals->_PTR_LOOP_1050_4a30 = 0x0;
    globals->_PTR_LOOP_1050_4a36 = 0x0;
    globals->PTR_LOOP_1050_492c  = 0xffff;
    globals->PTR_LOOP_1050_4932  = 0xffff;
    globals->PTR_LOOP_1050_4938  = 0xffff;
    globals->PTR_LOOP_1050_494a  = 0xffff;
    globals->PTR_LOOP_1050_4950  = 0xffff;
    globals->PTR_LOOP_1050_4a34  = 0xffff;
    globals->PTR_LOOP_1050_4a3a  = 0xffff;
    globals->_PTR_LOOP_1050_4a3c = 0x0;
    globals->_PTR_LOOP_1050_4a42 = 0x0;
    globals->PTR_LOOP_1050_4956  = 0x0;
    globals->PTR_LOOP_1050_495c  = 0x0;
    globals->PTR_LOOP_1050_4962  = 0x0;
    globals->PTR_LOOP_1050_4968  = 0x0;
    globals->PTR_LOOP_1050_496e  = 0x0;
    globals->PTR_LOOP_1050_4974  = 0x0;
    globals->PTR_LOOP_1050_497a  = 0x0;
    globals->PTR_LOOP_1050_4980  = 0x0;
    globals->PTR_LOOP_1050_4986  = 0x0;
    globals->PTR_LOOP_1050_498c  = 0x0;
    globals->PTR_LOOP_1050_4992  = 0x0;
    globals->PTR_LOOP_1050_4998  = 0x0;
    globals->PTR_LOOP_1050_4a04  = 0x0;
    globals->PTR_LOOP_1050_4a0a  = 0x0;
    globals->PTR_LOOP_1050_4a10  = 0x0;
    globals->PTR_LOOP_1050_4a16  = 0x0;
    globals->PTR_LOOP_1050_4a1c  = 0x0;
    globals->PTR_LOOP_1050_4a40  = 0x0;
    globals->PTR_LOOP_1050_4a46  = 0x0;
    globals->PTR_LOOP_1050_4a48  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_4a4a  = SEG_1050;
    globals->PTR_LOOP_1050_4a4c  = data_1050_4452;
    globals->PTR_LOOP_1050_4a4e  = data_1050_4448;//0x4448;
    globals->PTR_LOOP_1050_4a50  = SEG_1050;
    globals->PTR_LOOP_1050_4a52  = data_1050_4452;
    globals->_PTR_LOOP_1050_4a54 = 0x0;
    globals->_PTR_LOOP_1050_4a5a = 0x0;
    globals->_PTR_LOOP_1050_4a60 = 0x0;
    globals->_PTR_LOOP_1050_4a66 = 0x0;
    globals->_PTR_LOOP_1050_4a6c = 0x0;
    globals->_PTR_LOOP_1050_4a72 = 0x0;
    globals->_PTR_LOOP_1050_4a78 = 0x0;
    globals->_PTR_LOOP_1050_4a7e = 0x0;
    globals->_PTR_LOOP_1050_4a84 = 0x0;
    globals->_PTR_LOOP_1050_4a8a = 0x0;
    globals->_PTR_LOOP_1050_4a90 = 0x0;
    globals->_PTR_LOOP_1050_4a96 = 0x0;
    globals->_PTR_LOOP_1050_4a9c = 0x0;
    globals->_PTR_LOOP_1050_4aa2 = 0x0;
    globals->_PTR_LOOP_1050_4aa8 = 0x0;
    globals->_PTR_LOOP_1050_4aae = 0x0;
    globals->_PTR_LOOP_1050_4ab4 = 0x0;
    globals->_PTR_LOOP_1050_4aba = 0x0;
    globals->_PTR_LOOP_1050_4ac0 = 0x0;
    globals->_PTR_LOOP_1050_4ac6 = 0x0;
    globals->_PTR_LOOP_1050_4acc = 0x0;
    globals->_PTR_LOOP_1050_4ad2 = 0x0;
    globals->_PTR_LOOP_1050_4ad8 = 0x0;
    globals->_PTR_LOOP_1050_4ade = 0x0;
    globals->_PTR_LOOP_1050_4ae4 = 0x0;
    globals->_PTR_LOOP_1050_4aea = 0x0;
    globals->_PTR_LOOP_1050_4af0 = 0x0;
    globals->PTR_LOOP_1050_4a58  = 0x0;
    globals->PTR_LOOP_1050_4a5e  = 0x0;
    globals->PTR_LOOP_1050_4a64  = 0x0;
    globals->PTR_LOOP_1050_4a6a  = 0x0;
    globals->PTR_LOOP_1050_4a70  = 0x0;
    globals->PTR_LOOP_1050_4a76  = 0x0;
    globals->PTR_LOOP_1050_4a7c  = 0x0;
    globals->PTR_LOOP_1050_4a82  = 0x0;
    globals->PTR_LOOP_1050_4a88  = 0x0;
    globals->PTR_LOOP_1050_4a8e  = 0x0;
    globals->PTR_LOOP_1050_4a94  = 0x0;
    globals->PTR_LOOP_1050_4a9a  = 0x0;
    globals->PTR_LOOP_1050_4aa0  = 0x0;
    globals->PTR_LOOP_1050_4aa6  = 0x0;
    globals->PTR_LOOP_1050_4aac  = 0x0;
    globals->PTR_LOOP_1050_4ab2  = 0x0;
    globals->PTR_LOOP_1050_4ab8  = 0x0;
    globals->PTR_LOOP_1050_4abe  = 0x0;
    globals->PTR_LOOP_1050_4ac4  = 0x0;
    globals->PTR_LOOP_1050_4aca  = 0x0;
    globals->PTR_LOOP_1050_4ad0  = 0x0;
    globals->PTR_LOOP_1050_4ad6  = 0x0;
    globals->PTR_LOOP_1050_4adc  = 0x0;
    globals->PTR_LOOP_1050_4ae2  = 0x0;
    globals->PTR_LOOP_1050_4ae8  = 0x0;
    globals->PTR_LOOP_1050_4aee  = 0x0;
    globals->PTR_LOOP_1050_4af4  = 0x0;
//    pu_var_3                     = 0x4af6;
//    for(i_var_2 = 0x1b; i_var_2 != 0x0; i_var_2 = i_var_2 + -0x1)
//    {
//        pu_var_1  = pu_var_3;
//        pu_var_3  = pu_var_3 + 0x1;
//        *pu_var_1 = 0x0;
//    }
    globals->PTR_LOOP_1050_4b9c  = data_1050_4434;
    globals->_PTR_LOOP_1050_4b9e = 0x0;
    globals->_PTR_LOOP_1050_4ba4 = 0x0;
    globals->_PTR_LOOP_1050_4baa = 0x0;
    globals->PTR_LOOP_1050_4ba2  = 0xffff;
    globals->PTR_LOOP_1050_4ba8  = 0xffff;
    globals->PTR_LOOP_1050_4bae  = 0xffff;
    globals->_PTR_LOOP_1050_4bb0 = 0x0;
    globals->_PTR_LOOP_1050_4bb6 = 0x0;
    globals->PTR_LOOP_1050_4bbc  = data_1050_448e;//0x448e;
    globals->PTR_LOOP_1050_4bbe  = SEG_1050;
    globals->PTR_LOOP_1050_4bc2  = data_1050_448e;//0x448e;
    globals->PTR_LOOP_1050_4bc4  = SEG_1050;
    globals->PTR_LOOP_1050_4bc0  = data_1050_4494;
    globals->PTR_LOOP_1050_4bc6  = data_1050_4494;
    globals->PTR_LOOP_1050_4bc8  = data_1050_448e;//0x448e;
    globals->PTR_LOOP_1050_4bca  = SEG_1050;
    globals->PTR_LOOP_1050_4bcc  = data_1050_4494;
    globals->PTR_LOOP_1050_4bce  = data_1050_4482;//0x4482;
    globals->PTR_LOOP_1050_4bd0  = SEG_1050;
    globals->PTR_LOOP_1050_4bd4  = data_1050_4482;//0x4482;
    globals->PTR_LOOP_1050_4bd6  = SEG_1050;
    globals->PTR_LOOP_1050_4bd2  = data_1050_4486;
    globals->PTR_LOOP_1050_4bd8  = data_1050_4486;
    globals->PTR_LOOP_1050_4bda  = data_1050_4482;//0x4482;
    globals->PTR_LOOP_1050_4bdc  = SEG_1050;
    globals->PTR_LOOP_1050_4bde  = data_1050_4486;
    globals->PTR_LOOP_1050_4be2  = 0x0;
    globals->PTR_LOOP_1050_4be0  = 0x0;
    globals->PTR_LOOP_1050_4bb4  = 0x0;
    globals->PTR_LOOP_1050_4bba  = 0x0;
    globals->PTR_LOOP_1050_4be4  = 0x0;
    globals->PTR_LOOP_1050_4be6  = data_1050_44ac;//0x44ac;
    globals->PTR_LOOP_1050_4be8  = SEG_1050;
    globals->PTR_LOOP_1050_4bec  = data_1050_44ac;//0x44ac;
    globals->PTR_LOOP_1050_4bee  = SEG_1050;
    globals->PTR_LOOP_1050_4bea  = data_1050_44b2;
    globals->PTR_LOOP_1050_4bf0  = data_1050_44b2;
    globals->PTR_LOOP_1050_4bf2  = data_1050_44ac;//0x44ac;
    globals->PTR_LOOP_1050_4bf4  = SEG_1050;
    globals->PTR_LOOP_1050_4bf6  = data_1050_44b2;
    globals->PTR_LOOP_1050_4bf8  = data_1050_446a;//0x446a;
    globals->PTR_LOOP_1050_4bfa  = SEG_1050;
    globals->PTR_LOOP_1050_4bfe  = data_1050_446a;//0x446a;
    globals->PTR_LOOP_1050_4c00  = SEG_1050;
    globals->PTR_LOOP_1050_4bfc  = data_1050_446e;
    globals->PTR_LOOP_1050_4c02  = data_1050_446e;
    globals->PTR_LOOP_1050_4c04  = data_1050_446a;//0x446a;
    globals->PTR_LOOP_1050_4c06  = SEG_1050;
    globals->PTR_LOOP_1050_4c08  = data_1050_446e;
    globals->PTR_LOOP_1050_4c0a  = data_1050_448e;//0x448e;
    globals->PTR_LOOP_1050_4c0c  = SEG_1050;
    globals->PTR_LOOP_1050_4c0e  = data_1050_4494;
    globals->PTR_LOOP_1050_4c10  = data_1050_448e;//0x448e;
    globals->PTR_LOOP_1050_4c12  = SEG_1050;
    globals->PTR_LOOP_1050_4c14  = data_1050_4494;
    globals->PTR_LOOP_1050_4c16  = data_1050_44ac;//0x44ac;
    globals->PTR_LOOP_1050_4c18  = SEG_1050;
    globals->PTR_LOOP_1050_4c1a  = data_1050_44b2;
    globals->PTR_LOOP_1050_4c22  = data_1050_448e;//0x448e;
    globals->PTR_LOOP_1050_4c24  = SEG_1050;
    globals->PTR_LOOP_1050_4c26  = data_1050_4494;
    globals->_PTR_LOOP_1050_4c28 = 0x0;
    globals->_PTR_LOOP_1050_4c2e = 0x0;
    globals->_PTR_LOOP_1050_4c34 = 0x0;
    globals->_PTR_LOOP_1050_4c3a = 0x0;
    globals->_PTR_LOOP_1050_4c40 = 0x0;
    globals->_PTR_LOOP_1050_4c46 = 0x0;
    globals->_PTR_LOOP_1050_4c4c = 0x0;
    globals->_PTR_LOOP_1050_4c52 = 0x0;
    globals->PTR_LOOP_1050_4c1c  = data_1050_44ac;//0x44ac;
    globals->PTR_LOOP_1050_4c1e  = SEG_1050;
    globals->PTR_LOOP_1050_4c58  = data_1050_44ac;//0x44ac;
    globals->PTR_LOOP_1050_4c5a  = SEG_1050;
    globals->PTR_LOOP_1050_4c20  = data_1050_44b2;
    globals->PTR_LOOP_1050_4c5c  = data_1050_44b2;
    globals->PTR_LOOP_1050_4c5e  = data_1050_44ac;//0x44ac;
    globals->PTR_LOOP_1050_4c60  = SEG_1050;
    globals->PTR_LOOP_1050_4c62  = data_1050_44b2;
    globals->_PTR_LOOP_1050_4c64 = 0x0;
    globals->_PTR_LOOP_1050_4c6a = 0x0;
    globals->_PTR_LOOP_1050_4c70 = 0x0;
    globals->PTR_LOOP_1050_4c76  = data_1050_446a;//0x446a;
    globals->PTR_LOOP_1050_4c78  = SEG_1050;
    globals->PTR_LOOP_1050_4c7c  = data_1050_446a;//0x446a;
    globals->PTR_LOOP_1050_4c7e  = SEG_1050;
    globals->PTR_LOOP_1050_4c7a  = data_1050_446e;
    globals->PTR_LOOP_1050_4c80  = data_1050_446e;
    globals->_PTR_LOOP_1050_4c82 = 0x0;
    globals->_PTR_LOOP_1050_4c88 = 0x0;
    globals->PTR_LOOP_1050_4c2c  = 0xffff;
    globals->PTR_LOOP_1050_4c32  = 0xffff;
    globals->PTR_LOOP_1050_4c38  = 0xffff;
    globals->PTR_LOOP_1050_4c3e  = 0xffff;
    globals->PTR_LOOP_1050_4c44  = 0xffff;
    globals->PTR_LOOP_1050_4c4a  = 0xffff;
    globals->PTR_LOOP_1050_4c68  = 0xffff;
    globals->PTR_LOOP_1050_4c6e  = 0xffff;
    globals->PTR_LOOP_1050_4c74  = 0xffff;
    globals->PTR_LOOP_1050_4c86  = 0xffff;
    globals->PTR_LOOP_1050_4c8c  = 0xffff;
    globals->_PTR_LOOP_1050_4c8e = 0x0;
    globals->_PTR_LOOP_1050_4c94 = 0x0;
    globals->_PTR_LOOP_1050_4c9a = 0x0;
    globals->_PTR_LOOP_1050_4ca0 = 0x0;
    globals->_PTR_LOOP_1050_4ca6 = 0x0;
    globals->_PTR_LOOP_1050_4cac = 0x0;
    globals->_PTR_LOOP_1050_4cb2 = 0x0;
    globals->_PTR_LOOP_1050_4cb8 = 0x0;
    globals->_PTR_LOOP_1050_4cbe = 0x0;
    globals->_PTR_LOOP_1050_4cc4 = 0x0;
    globals->_PTR_LOOP_1050_4cca = 0x0;
    globals->_PTR_LOOP_1050_4cd0 = 0x0;
    globals->PTR_LOOP_1050_4cd6  = data_1050_4496;//0x4496;
    globals->PTR_LOOP_1050_4cd8  = SEG_1050;
    globals->PTR_LOOP_1050_4cdc  = data_1050_4496;//0x4496;
    globals->PTR_LOOP_1050_4cde  = SEG_1050;
    globals->PTR_LOOP_1050_4cda  = data_1050_44a2;
    globals->PTR_LOOP_1050_4ce0  = data_1050_44a2;
    globals->PTR_LOOP_1050_4ce2  = data_1050_4496;//0x4496;
    globals->PTR_LOOP_1050_4ce4  = SEG_1050;
    globals->PTR_LOOP_1050_4ce6  = data_1050_44a2;
    globals->PTR_LOOP_1050_4ce8  = data_1050_4496;//0x4496;
    globals->PTR_LOOP_1050_4cea  = SEG_1050;
    globals->PTR_LOOP_1050_4cec  = data_1050_44a2;
    globals->PTR_LOOP_1050_4cee  = data_1050_4496;//0x4496;
    globals->PTR_LOOP_1050_4cf0  = SEG_1050;
    globals->PTR_LOOP_1050_4cf2  = data_1050_44a2;
    globals->PTR_LOOP_1050_4cf4  = data_1050_44a4;//0x44a4;
    globals->PTR_LOOP_1050_4cf6  = SEG_1050;
    globals->PTR_LOOP_1050_4cfa  = data_1050_44a4;//0x44a4;
    globals->PTR_LOOP_1050_4cfc  = SEG_1050;
    globals->PTR_LOOP_1050_4cf8  = data_1050_44aa;
    globals->PTR_LOOP_1050_4cfe  = data_1050_44aa;
    globals->PTR_LOOP_1050_4d00  = data_1050_44a4;//0x44a4;
    globals->PTR_LOOP_1050_4d02  = SEG_1050;
    globals->PTR_LOOP_1050_4d04  = data_1050_44aa;
    globals->PTR_LOOP_1050_4d06  = data_1050_4496;//0x4496;
    globals->PTR_LOOP_1050_4d08  = SEG_1050;
    globals->PTR_LOOP_1050_4d0a  = data_1050_44a2;
    globals->PTR_LOOP_1050_4d0c  = data_1050_4496;//0x4496;
    globals->PTR_LOOP_1050_4d0e  = SEG_1050;
    globals->PTR_LOOP_1050_4d10  = data_1050_44a2;
    globals->PTR_LOOP_1050_4d12  = data_1050_4496;//0x4496;
    globals->PTR_LOOP_1050_4d14  = SEG_1050;
    globals->PTR_LOOP_1050_4d16  = data_1050_44a2;
    globals->PTR_LOOP_1050_4d18  = data_1050_4496;//0x4496;
    globals->PTR_LOOP_1050_4d1a  = SEG_1050;
    globals->PTR_LOOP_1050_4d1c  = data_1050_44a2;
    globals->PTR_LOOP_1050_4d1e  = data_1050_4482;//0x4482;
    globals->PTR_LOOP_1050_4d20  = SEG_1050;
    globals->PTR_LOOP_1050_4d24  = data_1050_4482;//0x4482;
    globals->PTR_LOOP_1050_4d26  = SEG_1050;
    globals->PTR_LOOP_1050_4d22  = data_1050_4486;
    globals->PTR_LOOP_1050_4d28  = data_1050_4486;
    globals->PTR_LOOP_1050_4d2a  = data_1050_44a4;//0x44a4;
    globals->PTR_LOOP_1050_4d2c  = SEG_1050;
    globals->PTR_LOOP_1050_4d2e  = data_1050_44aa;
    globals->PTR_LOOP_1050_4d30  = data_1050_44a4;//0x44a4;
    globals->PTR_LOOP_1050_4d32  = SEG_1050;
    globals->PTR_LOOP_1050_4d34  = data_1050_44aa;
    globals->PTR_LOOP_1050_4d36  = data_1050_44a4;//0x44a4;
    globals->PTR_LOOP_1050_4d38  = SEG_1050;
    globals->PTR_LOOP_1050_4d3a  = data_1050_44aa;
    globals->_PTR_LOOP_1050_4d3c = 0x0;
    globals->_PTR_LOOP_1050_4d42 = 0x0;
    globals->PTR_LOOP_1050_4c50  = 0x0;
    globals->PTR_LOOP_1050_4c56  = 0x0;
    globals->PTR_LOOP_1050_4c92  = 0x0;
    globals->PTR_LOOP_1050_4c98  = 0x0;
    globals->PTR_LOOP_1050_4c9e  = 0x0;
    globals->PTR_LOOP_1050_4ca4  = 0x0;
    globals->PTR_LOOP_1050_4caa  = 0x0;
    globals->PTR_LOOP_1050_4cb0  = 0x0;
    globals->PTR_LOOP_1050_4cb6  = 0x0;
    globals->PTR_LOOP_1050_4cbc  = 0x0;
    globals->PTR_LOOP_1050_4cc2  = 0x0;
    globals->PTR_LOOP_1050_4cc8  = 0x0;
    globals->PTR_LOOP_1050_4cce  = 0x0;
    globals->PTR_LOOP_1050_4cd4  = 0x0;
    globals->PTR_LOOP_1050_4d40  = 0x0;
    globals->PTR_LOOP_1050_4d46  = 0x0;
    globals->_PTR_LOOP_1050_4d48 = 0x0;
    globals->_PTR_LOOP_1050_4d4e = 0x0;
    globals->_PTR_LOOP_1050_4d54 = 0x0;
    globals->PTR_LOOP_1050_4d5a  = data_1050_44a4;//0x44a4;
    globals->PTR_LOOP_1050_4d5c  = SEG_1050;
    globals->PTR_LOOP_1050_4d5e  = data_1050_44aa;
    globals->PTR_LOOP_1050_4d60  = data_1050_4496;//0x4496;
    globals->PTR_LOOP_1050_4d62  = SEG_1050;
    globals->PTR_LOOP_1050_4d66  = data_1050_4496;//0x4496;
    globals->PTR_LOOP_1050_4d68  = SEG_1050;
    globals->PTR_LOOP_1050_4d64  = data_1050_44a2;
    globals->PTR_LOOP_1050_4d6a  = data_1050_44a2;
    globals->_PTR_LOOP_1050_4d6c = 0x0;
    globals->_PTR_LOOP_1050_4d72 = 0x0;
    globals->PTR_LOOP_1050_4d70  = 0xffff;
    globals->PTR_LOOP_1050_4d76  = 0xffff;
    globals->_PTR_LOOP_1050_4d78 = 0x0;
    globals->_PTR_LOOP_1050_4d7e = 0x0;
    globals->PTR_LOOP_1050_4d84  = data_1050_4496;//0x4496;
    globals->PTR_LOOP_1050_4d86  = SEG_1050;
    globals->PTR_LOOP_1050_4d88  = data_1050_44a2;
    globals->PTR_LOOP_1050_4d8a  = data_1050_4496;//0x4496;
    globals->PTR_LOOP_1050_4d8c  = SEG_1050;
    globals->PTR_LOOP_1050_4d8e  = data_1050_44a2;
    globals->_PTR_LOOP_1050_4d90 = 0x0;
    globals->_PTR_LOOP_1050_4d96 = 0x0;
    globals->_PTR_LOOP_1050_4d9c = 0x0;
    globals->_PTR_LOOP_1050_4da2 = 0x0;
    globals->_PTR_LOOP_1050_4da8 = 0x0;
    globals->_PTR_LOOP_1050_4dae = 0x0;
    globals->_PTR_LOOP_1050_4db4 = 0x0;
    globals->_PTR_LOOP_1050_4dba = 0x0;
    globals->_PTR_LOOP_1050_4dc0 = 0x0;
    globals->_PTR_LOOP_1050_4dc6 = 0x0;
    globals->_PTR_LOOP_1050_4dcc = 0x0;
    globals->_PTR_LOOP_1050_4dd2 = 0x0;
    globals->_PTR_LOOP_1050_4dd8 = 0x0;
    globals->_PTR_LOOP_1050_4dde = 0x0;
    globals->_PTR_LOOP_1050_4de4 = 0x0;
    globals->_PTR_LOOP_1050_4dea = 0x0;
    globals->_PTR_LOOP_1050_4df0 = 0x0;
    globals->_PTR_LOOP_1050_4df6 = 0x0;
    globals->_PTR_LOOP_1050_4dfc = 0x0;
    globals->_PTR_LOOP_1050_4e02 = 0x0;
    globals->_PTR_LOOP_1050_4e08 = 0x0;
    globals->_PTR_LOOP_1050_4e0e = 0x0;
    globals->_PTR_LOOP_1050_4e14 = 0x0;
    globals->_PTR_LOOP_1050_4e1a = 0x0;
    globals->_PTR_LOOP_1050_4e20 = 0x0;
    globals->_PTR_LOOP_1050_4e26 = 0x0;
    globals->_PTR_LOOP_1050_4e2c = 0x0;
    globals->PTR_LOOP_1050_4d4c  = 0x0;
    globals->PTR_LOOP_1050_4d52  = 0x0;
    globals->PTR_LOOP_1050_4d58  = 0x0;
    globals->PTR_LOOP_1050_4d7c  = 0x0;
    globals->PTR_LOOP_1050_4d82  = 0x0;
    globals->PTR_LOOP_1050_4d94  = 0x0;
    globals->PTR_LOOP_1050_4d9a  = 0x0;
    globals->PTR_LOOP_1050_4da0  = 0x0;
    globals->PTR_LOOP_1050_4da6  = 0x0;
    globals->PTR_LOOP_1050_4dac  = 0x0;
    globals->PTR_LOOP_1050_4db2  = 0x0;
    globals->PTR_LOOP_1050_4db8  = 0x0;
    globals->PTR_LOOP_1050_4dbe  = 0x0;
    globals->PTR_LOOP_1050_4dc4  = 0x0;
    globals->PTR_LOOP_1050_4dca  = 0x0;
    globals->PTR_LOOP_1050_4dd0  = 0x0;
    globals->PTR_LOOP_1050_4dd6  = 0x0;
    globals->PTR_LOOP_1050_4ddc  = 0x0;
    globals->PTR_LOOP_1050_4de2  = 0x0;
    globals->PTR_LOOP_1050_4de8  = 0x0;
    globals->PTR_LOOP_1050_4dee  = 0x0;
    globals->PTR_LOOP_1050_4df4  = 0x0;
    globals->PTR_LOOP_1050_4dfa  = 0x0;
    globals->PTR_LOOP_1050_4e00  = 0x0;
    globals->PTR_LOOP_1050_4e06  = 0x0;
    globals->PTR_LOOP_1050_4e0c  = 0x0;
    globals->PTR_LOOP_1050_4e12  = 0x0;
    globals->PTR_LOOP_1050_4e18  = 0x0;
    globals->PTR_LOOP_1050_4e1e  = 0x0;
    globals->PTR_LOOP_1050_4e24  = 0x0;
    globals->PTR_LOOP_1050_4e2a  = 0x0;
    globals->PTR_LOOP_1050_4e30  = 0x0;
//    pu_var_3                     = 0x4e32;
//    for(i_var_2 = 0x1b; i_var_2 != 0x0; i_var_2 = i_var_2 + -0x1)
//    {
//        pu_var_1  = pu_var_3;
//        pu_var_3  = pu_var_3 + 0x1;
//        *pu_var_1 = 0x0;
//    }
}
