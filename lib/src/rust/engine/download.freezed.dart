// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'download.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$ProgressEvent {





@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ProgressEvent);
}


@override
int get hashCode => runtimeType.hashCode;

@override
String toString() {
  return 'ProgressEvent()';
}


}

/// @nodoc
class $ProgressEventCopyWith<$Res>  {
$ProgressEventCopyWith(ProgressEvent _, $Res Function(ProgressEvent) __);
}


/// Adds pattern-matching-related methods to [ProgressEvent].
extension ProgressEventPatterns on ProgressEvent {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>({TResult Function( ProgressEvent_Started value)?  started,TResult Function( ProgressEvent_SegmentProgress value)?  segmentProgress,TResult Function( ProgressEvent_SegmentDone value)?  segmentDone,TResult Function( ProgressEvent_Merged value)?  merged,required TResult orElse(),}){
final _that = this;
switch (_that) {
case ProgressEvent_Started() when started != null:
return started(_that);case ProgressEvent_SegmentProgress() when segmentProgress != null:
return segmentProgress(_that);case ProgressEvent_SegmentDone() when segmentDone != null:
return segmentDone(_that);case ProgressEvent_Merged() when merged != null:
return merged(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>({required TResult Function( ProgressEvent_Started value)  started,required TResult Function( ProgressEvent_SegmentProgress value)  segmentProgress,required TResult Function( ProgressEvent_SegmentDone value)  segmentDone,required TResult Function( ProgressEvent_Merged value)  merged,}){
final _that = this;
switch (_that) {
case ProgressEvent_Started():
return started(_that);case ProgressEvent_SegmentProgress():
return segmentProgress(_that);case ProgressEvent_SegmentDone():
return segmentDone(_that);case ProgressEvent_Merged():
return merged(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>({TResult? Function( ProgressEvent_Started value)?  started,TResult? Function( ProgressEvent_SegmentProgress value)?  segmentProgress,TResult? Function( ProgressEvent_SegmentDone value)?  segmentDone,TResult? Function( ProgressEvent_Merged value)?  merged,}){
final _that = this;
switch (_that) {
case ProgressEvent_Started() when started != null:
return started(_that);case ProgressEvent_SegmentProgress() when segmentProgress != null:
return segmentProgress(_that);case ProgressEvent_SegmentDone() when segmentDone != null:
return segmentDone(_that);case ProgressEvent_Merged() when merged != null:
return merged(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>({TResult Function( BigInt? totalBytes,  BigInt numSegments)?  started,TResult Function( BigInt index,  BigInt bytesDownloaded)?  segmentProgress,TResult Function( BigInt index)?  segmentDone,TResult Function( BigInt totalBytes)?  merged,required TResult orElse(),}) {final _that = this;
switch (_that) {
case ProgressEvent_Started() when started != null:
return started(_that.totalBytes,_that.numSegments);case ProgressEvent_SegmentProgress() when segmentProgress != null:
return segmentProgress(_that.index,_that.bytesDownloaded);case ProgressEvent_SegmentDone() when segmentDone != null:
return segmentDone(_that.index);case ProgressEvent_Merged() when merged != null:
return merged(_that.totalBytes);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>({required TResult Function( BigInt? totalBytes,  BigInt numSegments)  started,required TResult Function( BigInt index,  BigInt bytesDownloaded)  segmentProgress,required TResult Function( BigInt index)  segmentDone,required TResult Function( BigInt totalBytes)  merged,}) {final _that = this;
switch (_that) {
case ProgressEvent_Started():
return started(_that.totalBytes,_that.numSegments);case ProgressEvent_SegmentProgress():
return segmentProgress(_that.index,_that.bytesDownloaded);case ProgressEvent_SegmentDone():
return segmentDone(_that.index);case ProgressEvent_Merged():
return merged(_that.totalBytes);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>({TResult? Function( BigInt? totalBytes,  BigInt numSegments)?  started,TResult? Function( BigInt index,  BigInt bytesDownloaded)?  segmentProgress,TResult? Function( BigInt index)?  segmentDone,TResult? Function( BigInt totalBytes)?  merged,}) {final _that = this;
switch (_that) {
case ProgressEvent_Started() when started != null:
return started(_that.totalBytes,_that.numSegments);case ProgressEvent_SegmentProgress() when segmentProgress != null:
return segmentProgress(_that.index,_that.bytesDownloaded);case ProgressEvent_SegmentDone() when segmentDone != null:
return segmentDone(_that.index);case ProgressEvent_Merged() when merged != null:
return merged(_that.totalBytes);case _:
  return null;

}
}

}

/// @nodoc


class ProgressEvent_Started extends ProgressEvent {
  const ProgressEvent_Started({this.totalBytes, required this.numSegments}): super._();
  

 final  BigInt? totalBytes;
 final  BigInt numSegments;

/// Create a copy of ProgressEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ProgressEvent_StartedCopyWith<ProgressEvent_Started> get copyWith => _$ProgressEvent_StartedCopyWithImpl<ProgressEvent_Started>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ProgressEvent_Started&&(identical(other.totalBytes, totalBytes) || other.totalBytes == totalBytes)&&(identical(other.numSegments, numSegments) || other.numSegments == numSegments));
}


@override
int get hashCode => Object.hash(runtimeType,totalBytes,numSegments);

@override
String toString() {
  return 'ProgressEvent.started(totalBytes: $totalBytes, numSegments: $numSegments)';
}


}

/// @nodoc
abstract mixin class $ProgressEvent_StartedCopyWith<$Res> implements $ProgressEventCopyWith<$Res> {
  factory $ProgressEvent_StartedCopyWith(ProgressEvent_Started value, $Res Function(ProgressEvent_Started) _then) = _$ProgressEvent_StartedCopyWithImpl;
@useResult
$Res call({
 BigInt? totalBytes, BigInt numSegments
});




}
/// @nodoc
class _$ProgressEvent_StartedCopyWithImpl<$Res>
    implements $ProgressEvent_StartedCopyWith<$Res> {
  _$ProgressEvent_StartedCopyWithImpl(this._self, this._then);

  final ProgressEvent_Started _self;
  final $Res Function(ProgressEvent_Started) _then;

/// Create a copy of ProgressEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? totalBytes = freezed,Object? numSegments = null,}) {
  return _then(ProgressEvent_Started(
totalBytes: freezed == totalBytes ? _self.totalBytes : totalBytes // ignore: cast_nullable_to_non_nullable
as BigInt?,numSegments: null == numSegments ? _self.numSegments : numSegments // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class ProgressEvent_SegmentProgress extends ProgressEvent {
  const ProgressEvent_SegmentProgress({required this.index, required this.bytesDownloaded}): super._();
  

 final  BigInt index;
 final  BigInt bytesDownloaded;

/// Create a copy of ProgressEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ProgressEvent_SegmentProgressCopyWith<ProgressEvent_SegmentProgress> get copyWith => _$ProgressEvent_SegmentProgressCopyWithImpl<ProgressEvent_SegmentProgress>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ProgressEvent_SegmentProgress&&(identical(other.index, index) || other.index == index)&&(identical(other.bytesDownloaded, bytesDownloaded) || other.bytesDownloaded == bytesDownloaded));
}


@override
int get hashCode => Object.hash(runtimeType,index,bytesDownloaded);

@override
String toString() {
  return 'ProgressEvent.segmentProgress(index: $index, bytesDownloaded: $bytesDownloaded)';
}


}

/// @nodoc
abstract mixin class $ProgressEvent_SegmentProgressCopyWith<$Res> implements $ProgressEventCopyWith<$Res> {
  factory $ProgressEvent_SegmentProgressCopyWith(ProgressEvent_SegmentProgress value, $Res Function(ProgressEvent_SegmentProgress) _then) = _$ProgressEvent_SegmentProgressCopyWithImpl;
@useResult
$Res call({
 BigInt index, BigInt bytesDownloaded
});




}
/// @nodoc
class _$ProgressEvent_SegmentProgressCopyWithImpl<$Res>
    implements $ProgressEvent_SegmentProgressCopyWith<$Res> {
  _$ProgressEvent_SegmentProgressCopyWithImpl(this._self, this._then);

  final ProgressEvent_SegmentProgress _self;
  final $Res Function(ProgressEvent_SegmentProgress) _then;

/// Create a copy of ProgressEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? index = null,Object? bytesDownloaded = null,}) {
  return _then(ProgressEvent_SegmentProgress(
index: null == index ? _self.index : index // ignore: cast_nullable_to_non_nullable
as BigInt,bytesDownloaded: null == bytesDownloaded ? _self.bytesDownloaded : bytesDownloaded // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class ProgressEvent_SegmentDone extends ProgressEvent {
  const ProgressEvent_SegmentDone({required this.index}): super._();
  

 final  BigInt index;

/// Create a copy of ProgressEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ProgressEvent_SegmentDoneCopyWith<ProgressEvent_SegmentDone> get copyWith => _$ProgressEvent_SegmentDoneCopyWithImpl<ProgressEvent_SegmentDone>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ProgressEvent_SegmentDone&&(identical(other.index, index) || other.index == index));
}


@override
int get hashCode => Object.hash(runtimeType,index);

@override
String toString() {
  return 'ProgressEvent.segmentDone(index: $index)';
}


}

/// @nodoc
abstract mixin class $ProgressEvent_SegmentDoneCopyWith<$Res> implements $ProgressEventCopyWith<$Res> {
  factory $ProgressEvent_SegmentDoneCopyWith(ProgressEvent_SegmentDone value, $Res Function(ProgressEvent_SegmentDone) _then) = _$ProgressEvent_SegmentDoneCopyWithImpl;
@useResult
$Res call({
 BigInt index
});




}
/// @nodoc
class _$ProgressEvent_SegmentDoneCopyWithImpl<$Res>
    implements $ProgressEvent_SegmentDoneCopyWith<$Res> {
  _$ProgressEvent_SegmentDoneCopyWithImpl(this._self, this._then);

  final ProgressEvent_SegmentDone _self;
  final $Res Function(ProgressEvent_SegmentDone) _then;

/// Create a copy of ProgressEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? index = null,}) {
  return _then(ProgressEvent_SegmentDone(
index: null == index ? _self.index : index // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

/// @nodoc


class ProgressEvent_Merged extends ProgressEvent {
  const ProgressEvent_Merged({required this.totalBytes}): super._();
  

 final  BigInt totalBytes;

/// Create a copy of ProgressEvent
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$ProgressEvent_MergedCopyWith<ProgressEvent_Merged> get copyWith => _$ProgressEvent_MergedCopyWithImpl<ProgressEvent_Merged>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is ProgressEvent_Merged&&(identical(other.totalBytes, totalBytes) || other.totalBytes == totalBytes));
}


@override
int get hashCode => Object.hash(runtimeType,totalBytes);

@override
String toString() {
  return 'ProgressEvent.merged(totalBytes: $totalBytes)';
}


}

/// @nodoc
abstract mixin class $ProgressEvent_MergedCopyWith<$Res> implements $ProgressEventCopyWith<$Res> {
  factory $ProgressEvent_MergedCopyWith(ProgressEvent_Merged value, $Res Function(ProgressEvent_Merged) _then) = _$ProgressEvent_MergedCopyWithImpl;
@useResult
$Res call({
 BigInt totalBytes
});




}
/// @nodoc
class _$ProgressEvent_MergedCopyWithImpl<$Res>
    implements $ProgressEvent_MergedCopyWith<$Res> {
  _$ProgressEvent_MergedCopyWithImpl(this._self, this._then);

  final ProgressEvent_Merged _self;
  final $Res Function(ProgressEvent_Merged) _then;

/// Create a copy of ProgressEvent
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') $Res call({Object? totalBytes = null,}) {
  return _then(ProgressEvent_Merged(
totalBytes: null == totalBytes ? _self.totalBytes : totalBytes // ignore: cast_nullable_to_non_nullable
as BigInt,
  ));
}


}

// dart format on
