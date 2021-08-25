# Model report for file:///tmp/top-repos-quality-repos-3mj46btr/tiny-tools.git HEAD 2af2c0ed405ded5013d1902ca048a64bd9bb4f39

### Dump

```json
{'created_at': '2021-08-21 09:54:56',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.0 kB',
 'tags': [],
 'uuid': 'c4d3c3b0-b814-43f1-b615-6cf99e1c284b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-3mj46btr/tiny-tools.git 2af2c0ed405ded5013d1902ca048a64bd9bb4f39

# javascript
11 rules, avg.len. 6.8
## train
PPCR: 0.679511
### report
macro
{'f1-score': 0.6661530656951603,
 'precision': 0.666778416259293,
 'recall': 0.6661148258492011,
 'support': 4832}
micro
{'f1-score': 0.9590231788079471,
 'precision': 0.9590231788079471,
 'recall': 0.9590231788079471,
 'support': 4832}
weighted
{'f1-score': 0.9560487420028765,
 'precision': 0.9533383134539434,
 'recall': 0.9590231788079471,
 'support': 4832}
### report_full
macro
{'f1-score': 0.5796221000595351,
 'precision': 0.666778416259293,
 'recall': 0.5308723085864769,
 'support': 7111}
micro
{'f1-score': 0.776019425604957,
 'precision': 0.9590231788079471,
 'recall': 0.6516664322880045,
 'support': 7111}
weighted
{'f1-score': 0.7317513360964594,
 'precision': 0.8741762891558127,
 'recall': 0.6516664322880045,
 'support': 7111}
## test
PPCR: 0.670216
### report
macro
{'f1-score': 0.6302448095884144,
 'precision': 0.6405450891952232,
 'recall': 0.6221414643410902,
 'support': 40030}
micro
{'f1-score': 0.9161129153135149,
 'precision': 0.9161129153135149,
 'recall': 0.9161129153135149,
 'support': 40030}
weighted
{'f1-score': 0.9117917414729646,
 'precision': 0.9092156363181569,
 'recall': 0.9161129153135149,
 'support': 40030}
### report_full
macro
{'f1-score': 0.5523054710031766,
 'precision': 0.6405450891952232,
 'recall': 0.4978086705917865,
 'support': 59727}
micro
{'f1-score': 0.7352266006395541,
 'precision': 0.9161129153135149,
 'recall': 0.6139936712039781,
 'support': 59727}
weighted
{'f1-score': 0.6893027736895071,
 'precision': 0.8313175986117426,
 'recall': 0.6139936712039781,
 'support': 59727}
```

## javascript
### Summary
7 rules, avg.len. 8.0

| | |
|-|-|
|Min support|102|
|Max support|1202|
|Min confidence|0.9685714244842529|
|Max confidence|0.9961538314819336|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = Identifier<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 1147.` |
| 2 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.969. Support: 175.` |
| 3 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 204.` |
| 4 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {,, {}<br>	∧ +1.reserved not in {{, }}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.996. Support: 130.` |
| 5 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.roles in {BLOCK}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 102.` |
| 6 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.length ≥ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {COMMENT} and not in {STRING}<br>	∧ ^1.roles not in {MAP, OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 126.` |
| 7 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {Identifier, StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.length ≤ 1<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {MAP, OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 1202.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.0, "max_conf": 0.9961538314819336, "max_support": 1202, "min_conf": 0.9685714244842529, "min_support": 102, "num_rules": 7}}
```
</details>
